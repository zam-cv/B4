use crate::socket::context::Context;
use anyhow::Result;
use macros::getter;
use rand::Rng;

#[getter]
fn __get_money(ctx: &mut Context) -> Result<String> {
    Ok(ctx.player.balance_cash.to_string())
}

#[getter]
fn __get_value_random(_: &mut Context, first: u32, second: u32) -> Result<String> {
    Ok(rand::thread_rng().gen_range(first..second).to_string())
}
