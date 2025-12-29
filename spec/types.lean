-- Primitive opaque types
constant Address   : Type
constant Signature : Type
constant Hash32    : Type

-- Basic numeric aliases
abbrev Amount      := Nat
abbrev Nonce       := Nat
abbrev BlockHeight := Nat
abbrev ChainId     := Nat

-- Fixed chain id
def CHAIN_ID : ChainId := 1

-- State model
structure State where
  balances : Address → Amount
  nonces   : Address → Nonce

