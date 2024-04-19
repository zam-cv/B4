use crate::socket::context::Context;
use anyhow::Result;
use macros::handler;

#[handler]
fn __increment_money(ctx: &mut Context, amount: i32) -> Result<()> {
    ctx.player.balance_cash += amount;
    Ok(())
}

#[handler]
fn __decrement_money(ctx: &mut Context, amount: i32) -> Result<()> {
    ctx.player.balance_cash -= amount;
    Ok(())
}