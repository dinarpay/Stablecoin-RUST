// Stablecoin to payment

#![no_std]
#![no_main]

imports!();

const NAME:     &[u8]   = b"American Dolar USAD";
const SYMBOL:   &[u8]   = b"USAD";
const DECIMALS: usize   = 18;
const COMISION: &[u8]   = 10/100;