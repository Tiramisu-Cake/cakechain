import Cakechain.types

noncomputable def update_balance (S : State) (a : Address) (v : Amount) : State :=
  { S with 
    balances := fun x => if x = a then v else S.balances x }

noncomputable def update_nonce (S : State) (a : Address) (v : Nonce)  : State := 
  { S with 
    nonces := fun x => if x = a then v else S.nonces x }

theorem update_balance_same :
  (update_balance S a v).balances a = v := by simp[update_balance] 

theorem update_balance_other : 
    a ≠ b → (update_balance S a v).balances b = S.balances b := by
    intro hne
    have : b ≠ a := by exact Ne.symm hne
    simp [update_balance, this]

theorem update_nonce_same :
  (update_nonce S a v).nonces a = v := by simp[update_nonce] 

theorem update_nonce_other : 
    a ≠ b → (update_nonce S a v).nonces b = S.nonces b := by
    intro hne
    have : b ≠ a := by exact Ne.symm hne
    simp [update_nonce, this]


theorem state_ext
  (S₁ S₂ : State)
  (h_balances : ∀ a : Address, S₁.balances a = S₂.balances a)
  (h_nonces   : ∀ a : Address, S₁.nonces a   = S₂.nonces a) :
  S₁ = S₂ := by
    cases S₁ with
  | mk b₁ n₁ =>
    cases S₂ with
    | mk b₂ n₂ =>
      have hb : b₁ = b₂ := by
        funext a
        exact h_balances a
      have hn : n₁ = n₂ := by
        funext a
        exact h_nonces a
      cases hb
      cases hn
      rfl


