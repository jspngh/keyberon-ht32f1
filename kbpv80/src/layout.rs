use keyberon::action::l;

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<8, 15, 3, ()> = keyberon::layout::layout! {
    {
        [No     LShift   RShift No      Down    Left    Right   Up],
        [No     LGui     RGui   {l(2)}  RAlt    LAlt    No      No],
        [Grave  Escape   No     No      RCtrl   No      LCtrl   No],
        [Q      Tab      A      No      Z       No      No      Kb1],
        [W      {l(1)}   S      No      X       No      No      Kb2],
        [E      No       D      No      C       No      No      Kb3],
        [R      T        F      G       V       B       Kb5     Kb4],
        [U      Y        J      H       M       N       Kb6     Kb7],
        [I      RBracket K      No      Comma   No      Equal   Kb8],
        [O      No       L      No      Dot     No      No      Kb9],
        [No     BSpace   Bslash Space   Enter   Insert  Delete  Home],
        [F1     F2       F3     F4      F5      F6      F7      F8],
        [No     F9       F10    F11     F12     End     PgUp    PgDown],
        [P      LBracket SColon Quote   No      Slash   Minus   Kb0],
        [No     PScreen  ScrollLock Pause No    No      No      No],
    }
    {
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        Down   Left    t       t       t       t],
        [t      t        Up     t       t       t       t       t],
        [t      t        Right  t       t       t       t       t],
        [t      Delete   t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
    }
    {
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      CapsLock t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
    }
};

