pub fn amount_to_minor_unit(amount: f64) -> String {
    let amount_in_pesewas: f64 = amount * 100.00;
    let rounded_amount_to_int = amount_in_pesewas.round() as i64;
    let formatted_amount = format!("{:012}", rounded_amount_to_int);

    return formatted_amount;
}
