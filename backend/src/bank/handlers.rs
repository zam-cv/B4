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

#[handler]
fn __drop_money(ctx: &mut Context) -> Result<()> {
    ctx.player.balance_cash = 0;
    Ok(())
}

#[handler]
fn __duplicate_money(ctx: &mut Context) -> Result<()> {
    ctx.player.balance_cash *= 2;
    Ok(())
}
