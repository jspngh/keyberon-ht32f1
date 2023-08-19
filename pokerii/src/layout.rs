use keyberon::action::l;

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<8, 9, 3, ()> = keyberon::layout::layout! {
    {
        [Escape Kb1     Kb2     Kb3     Kb4     Kb5     Kb6     Kb7],
        [Tab    Q       W       E       R       T       Y       U],
        [{l(1)} A       S       D       F       G       H       J],
        [LShift Z       X       C       V       B       N       M],
        [LCtrl  LAlt    LGui    Kb8     Kb9     Kb0     Minus   BSpace],
        [No     No      Equal   I       O       P       LBracket Bslash],
        [No     No      RBracket K      L       SColon  Quote   Enter],
        [No     No      Space   Comma   Dot     Slash   RShift  No],
        [No     No      No      {l(2)}  RAlt    RGui    No      RCtrl],
    }
    {
        [Grave  F1       F2     F3      F4      F5      F6      F7],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       Left    Down],
        [t      t        t      t       t       t       t       t],
        [t      t        t      F8      F9      F10     F11     Delete],
        [t      t        F12    t       t       t       t       t],
        [t      t        t      Up      Right   t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
    }
    {
        [t      t        t      t       t       t       t       t],
        [t      t        Up     t       t       t       t       t],
        [CapsLock Left   Down   Right   t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
        [t      t        t      t       t       t       t       t],
    }
};
