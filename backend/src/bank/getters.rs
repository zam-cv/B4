use crate::socket::{context::Context, state::Duration};
use anyhow::Result;
use macros::getter;
use rand::Rng;

fn get_random(first: u32, second: u32) -> u32 {
    rand::thread_rng().gen_range(first..=second)
}

#[getter]
fn __get_money(ctx: &mut Context) -> Result<String> {
    Ok(ctx.player.balance_cash.to_string())
}

#[getter]
fn __get_value_random(_: &mut Context, first: u32, second: u32) -> Result<String> {
    Ok(get_random(first, second).to_string())
}

#[getter]
fn __robar(ctx: &mut Context) -> Result<String> {
    let porcentaje: u32 = get_random(1, 100);
    let cantidad_robada = (ctx.player.balance_cash as f64 * (porcentaje as f64 / 100.0)) as u32;
    ctx.player.balance_cash -= cantidad_robada as i32;
    Ok(cantidad_robada.to_string())
}

#[getter]
fn __premio_mayor(ctx: &mut Context) -> Result<String> {
    let ganancia = ctx.player.max_change / 100;
    ctx.player.balance_cash += ganancia as i32;
    Ok(ganancia.to_string())
}

#[getter]
fn __get_time(ctx: &mut Context) -> Result<String> {
    Ok(match ctx.cycle_data.duration {
        Duration::OneMonth => "1 mes",
        Duration::SixMonths => "6 meses",
        Duration::OneYear => "1 año",
    }
    .to_string())
}

#[getter]
fn __get_personal_expenses(ctx: &mut Context) -> Result<String> {
    let months = ctx.cycle_data.duration.to_months();
    let quantity = months * 30 * get_random(200, 400); // months * days * random
    Ok(quantity.to_string())
}
