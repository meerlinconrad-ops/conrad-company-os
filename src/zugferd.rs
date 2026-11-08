#![allow(dead_code)]
use crate::models::{Contact, Invoice, InvoiceItem};

/// Escapes XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Generates official EN 16931 / ZUGFeRD 2.2 / XRechnung 3.0 CrossIndustryInvoice (CII) XML
pub fn generate_xrechnung_xml(
    invoice: &Invoice,
    items: &[InvoiceItem],
    seller_name: &str,
    seller_vat_id: &str,
    seller_iban: &str,
    seller_bic: &str,
    buyer: Option<&Contact>,
) -> String {
    let issue_date_compact = invoice.issued_date.replace('-', "");
    let due_date_compact = invoice.due_date.replace('-', "");

    let buyer_name = buyer
        .map(|b| b.name.as_str())
        .or(invoice.contact_name.as_deref())
        .unwrap_or("Kunde");
    let buyer_company = buyer.and_then(|b| b.company.as_deref()).unwrap_or(buyer_name);
    let buyer_email = buyer.and_then(|b| b.email.as_deref()).unwrap_or("rechnung@kunde.de");

    let is_storno = invoice.invoice_number.starts_with("ST-") || invoice.status == "cancelled";
    let type_code = if is_storno { "381" } else { "380" }; // 380 = Handelsrechnung, 381 = Gutschrift/Storno

    let mut line_items_xml = String::new();
    for (idx, item) in items.iter().enumerate() {
        let line_num = idx + 1;
        let unit_price_eur = item.unit_price_cents as f64 / 100.0;
        let line_total_eur = item.total_cents as f64 / 100.0;

        line_items_xml.push_str(&format!(
            r#"
        <ram:IncludedSupplyChainTradeLineItem>
            <ram:AssociatedDocumentLineDocument>
                <ram:LineID>{line_num}</ram:LineID>
            </ram:AssociatedDocumentLineDocument>
            <ram:SpecifiedTradeProduct>
                <ram:Name>{name}</ram:Name>
            </ram:SpecifiedTradeProduct>
            <ram:SpecifiedLineTradeAgreement>
                <ram:NetPriceProductTradePrice>
                    <ram:ChargeAmount currencyID="EUR">{unit_price:.2}</ram:ChargeAmount>
                </ram:NetPriceProductTradePrice>
            </ram:SpecifiedLineTradeAgreement>
            <ram:SpecifiedLineTradeDelivery>
                <ram:BilledQuantity unitCode="C62">{qty:.2}</ram:BilledQuantity>
            </ram:SpecifiedLineTradeDelivery>
            <ram:SpecifiedLineTradeSettlement>
                <ram:ApplicableTradeTax>
                    <ram:TypeCode>VAT</ram:TypeCode>
                    <ram:CategoryCode>S</ram:CategoryCode>
                    <ram:RateApplicablePercent>19.00</ram:RateApplicablePercent>
                </ram:ApplicableTradeTax>
                <ram:SpecifiedTradeSettlementLineMonetarySummation>
                    <ram:LineTotalAmount currencyID="EUR">{total:.2}</ram:LineTotalAmount>
                </ram:SpecifiedTradeSettlementLineMonetarySummation>
            </ram:SpecifiedLineTradeSettlement>
        </ram:IncludedSupplyChainTradeLineItem>"#,
            line_num = line_num,
            name = escape_xml(&item.description),
            unit_price = unit_price_eur,
            qty = item.quantity,
            total = line_total_eur,
        ));
    }

    let net_eur = invoice.net_cents as f64 / 100.0;
    let vat_eur = invoice.vat_cents as f64 / 100.0;
    let gross_eur = invoice.gross_cents as f64 / 100.0;

    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rsm:CrossIndustryInvoice xmlns:rsm="urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100"
    xmlns:ram="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100"
    xmlns:qdt="urn:un:unece:uncefact:data:standard:QualifiedDataType:100"
    xmlns:udt="urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100">
    <rsm:ExchangedDocumentContext>
        <ram:GuidelineSpecifiedDocumentContextParameter>
            <ram:ID>urn:cen.eu:en16931:2017#compliant#urn:xeinkauf.de:kosit:xrechnung_3.0</ram:ID>
        </ram:GuidelineSpecifiedDocumentContextParameter>
    </rsm:ExchangedDocumentContext>
    <rsm:ExchangedDocument>
        <ram:ID>{invoice_number}</ram:ID>
        <ram:TypeCode>{type_code}</ram:TypeCode>
        <ram:IssueDateTime>
            <udt:DateTimeString format="102">{issue_date_compact}</udt:DateTimeString>
        </ram:IssueDateTime>
    </rsm:ExchangedDocument>
    <rsm:SupplyChainTradeTransaction>
        {line_items_xml}
        <ram:ApplicableHeaderTradeAgreement>
            <ram:BuyerReference>LEITWEG-ID-08222000-0001</ram:BuyerReference>
            <ram:SellerTradeParty>
                <ram:Name>{seller_name}</ram:Name>
                <ram:SpecifiedTaxRegistration>
                    <ram:ID schemeID="VA">{seller_vat_id}</ram:ID>
                </ram:SpecifiedTaxRegistration>
            </ram:SellerTradeParty>
            <ram:BuyerTradeParty>
                <ram:Name>{buyer_company}</ram:Name>
                <ram:DefinedTradeContact>
                    <ram:PersonName>{buyer_name}</ram:PersonName>
                    <ram:EmailURIUniversalCommunication>
                        <ram:URIID>{buyer_email}</ram:URIID>
                    </ram:EmailURIUniversalCommunication>
                </ram:DefinedTradeContact>
            </ram:BuyerTradeParty>
        </ram:ApplicableHeaderTradeAgreement>
        <ram:ApplicableHeaderTradeDelivery>
            <ram:ActualDeliverySupplyChainEvent>
                <ram:OccurrenceDateTime>
                    <udt:DateTimeString format="102">{issue_date_compact}</udt:DateTimeString>
                </ram:OccurrenceDateTime>
            </ram:ActualDeliverySupplyChainEvent>
        </ram:ApplicableHeaderTradeDelivery>
        <ram:ApplicableHeaderTradeSettlement>
            <ram:PaymentReference>{invoice_number}</ram:PaymentReference>
            <ram:InvoiceCurrencyCode>EUR</ram:InvoiceCurrencyCode>
            <ram:SpecifiedTradeSettlementPaymentMeans>
                <ram:TypeCode>58</ram:TypeCode> <!-- SEPA Credit Transfer -->
                <ram:PayeePartyCreditorFinancialAccount>
                    <ram:IBANID>{seller_iban}</ram:IBANID>
                </ram:PayeePartyCreditorFinancialAccount>
                <ram:PayeeSpecifiedCreditorFinancialInstitution>
                    <ram:BICID>{seller_bic}</ram:BICID>
                </ram:PayeeSpecifiedCreditorFinancialInstitution>
            </ram:SpecifiedTradeSettlementPaymentMeans>
            <ram:ApplicableTradeTax>
                <ram:CalculatedAmount currencyID="EUR">{vat_eur:.2}</ram:CalculatedAmount>
                <ram:TypeCode>VAT</ram:TypeCode>
                <ram:BasisAmount currencyID="EUR">{net_eur:.2}</ram:BasisAmount>
                <ram:CategoryCode>S</ram:CategoryCode>
                <ram:RateApplicablePercent>19.00</ram:RateApplicablePercent>
            </ram:ApplicableTradeTax>
            <ram:SpecifiedTradePaymentTerms>
                <ram:DueDateDateTime>
                    <udt:DateTimeString format="102">{due_date_compact}</udt:DateTimeString>
                </ram:DueDateDateTime>
            </ram:SpecifiedTradePaymentTerms>
            <ram:SpecifiedTradeSettlementHeaderMonetarySummation>
                <ram:LineTotalAmount currencyID="EUR">{net_eur:.2}</ram:LineTotalAmount>
                <ram:TaxBasisTotalAmount currencyID="EUR">{net_eur:.2}</ram:TaxBasisTotalAmount>
                <ram:TaxTotalAmount currencyID="EUR">{vat_eur:.2}</ram:TaxTotalAmount>
                <ram:GrandTotalAmount currencyID="EUR">{gross_eur:.2}</ram:GrandTotalAmount>
                <ram:DuePayableAmount currencyID="EUR">{gross_eur:.2}</ram:DuePayableAmount>
            </ram:SpecifiedTradeSettlementHeaderMonetarySummation>
        </ram:ApplicableHeaderTradeSettlement>
    </rsm:SupplyChainTradeTransaction>
</rsm:CrossIndustryInvoice>"#,
        invoice_number = escape_xml(&invoice.invoice_number),
        type_code = type_code,
        issue_date_compact = issue_date_compact,
        line_items_xml = line_items_xml,
        seller_name = escape_xml(seller_name),
        seller_vat_id = escape_xml(seller_vat_id),
        buyer_company = escape_xml(buyer_company),
        buyer_name = escape_xml(buyer_name),
        buyer_email = escape_xml(buyer_email),
        seller_iban = seller_iban.replace(' ', ""),
        seller_bic = seller_bic.replace(' ', ""),
        due_date_compact = due_date_compact,
        net_eur = net_eur,
        vat_eur = vat_eur,
        gross_eur = gross_eur
    )
}
