macro_rules! op_trait_impl {
    (
        $trait:ident, $name:ident, $($ab:lifetime,)*
        $slf:ident, $lt:ty, $rhs:ident, $rt:ty,
        $ot:ty, $($meta:meta,)* $code:block
    ) => {
        impl<$($ab,)*> $trait<$rt> for $lt {
            type Output = $ot;

            $(#[$meta])*
            fn $name($slf, $rhs:$rt) -> Self::Output $code
        }
    };
    (
        $trait:ident, $name:ident, $($ab:lifetime,)*
        $slf:ident, $lt:ty, $rhs:ident, $rt:ty,
        $($meta:meta,)* $code:block
    ) => {
        impl<$($ab,)*> $trait<$rt> for $lt {
            $(#[$meta])*
            fn $name(&mut $slf, $rhs:$rt) $code
        }
    };
    (
        $trait:ident, $name:ident, $($a:lifetime,)?
        $slf:ident, $t:ty, $ot:ty,
        $($meta:meta,)* $code:block
    ) => {
        impl<$($a)?> $trait for $t {
            type Output = $ot;

            $(#[$meta])*
            fn $name($slf) -> Self::Output $code
        }
    };
}

macro_rules! binary_ops {
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$($ab:lifetime),+>(
            $slf:ident: &$a:lifetime $lt:ty, $rhs:ident: &$b:lifetime $rt:ty) -> $ot:ty
        $code:block
    ) => {
        op_trait_impl!($trait, $name, $($ab),+, $slf, &$a $lt, $rhs, &$b $rt, $ot, $($meta,)* $code);
        op_trait_impl!($trait, $name, $a, $slf, &$a $lt, $rhs, $rt, $ot, $($meta,)* { $slf.$name(&$rhs) });
        op_trait_impl!($trait, $name, $b, $slf, $lt, $rhs, &$b $rt, $ot, $($meta,)* { (&$slf).$name($rhs) });
        op_trait_impl!($trait, $name, $slf, $lt, $rhs, $rt, $ot, $($meta,)* { (&$slf).$name(&$rhs) });
    };
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$($ab:lifetime),+>(
            $slf:ident: &$a:lifetime $lt:ty, $rhs:ident: &$b:lifetime $rt:ty) -> $ot:ty
        $code:block
        $(
            $(#[$metas:meta])*
            fn $traits:ident::$names:ident<$($abs:lifetime),+>(
                $slfs:ident: &$as:lifetime $lts:ty, $rhss:ident: &$bs:lifetime $rts:ty) -> $ots:ty
            $codes:block
        )+
    ) => {
        binary_ops! {
            $(#[$meta])*
            fn $trait::$name<$($ab),+>($slf: &$a $lt, $rhs: &$b $rt) -> $ot
            $code
        }

        binary_ops! {
            $(
                $(#[$metas])*
                fn $traits::$names<$($abs),+>($slfs: &$as $lts, $rhss: &$bs $rts) -> $ots
                $codes
            )+
        }
    };
}

macro_rules! assign_ops {
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$a:lifetime>(
            $slf:ident: &mut $lt:ty, $rhs:ident: &$aa:lifetime $rt:ty) $code:block
    ) => {
        op_trait_impl!($trait, $name, $a, $slf, $lt, $rhs, &$aa $rt, $($meta,)* $code);
        op_trait_impl!($trait, $name, $a, $slf, $lt, $rhs, $rt, $($meta,)* { $slf.$name(&$rhs); });
    };
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident<$a:lifetime>(
            $slf:ident: &mut $lt:ty, $rhs:ident: &$aa:lifetime $rt:ty) $code:block
        $(
            $(#[$metas:meta])*
            fn $traits:ident::$names:ident<$as:lifetime>(
                $slfs:ident: &mut $lts:ty, $rhss:ident: &$aas:lifetime $rts:ty) $codes:block
        )+
    ) => {
        assign_ops! {
            $(#[$meta])*
            fn $trait::$name<$a>($slf: &mut $lt, $rhs: &$aa $rt) $code
        }

        assign_ops! {
            $(
                $(#[$metas])*
                fn $traits::$names<$as>($slfs: &mut $lts, $rhss: &$aas $rts) $codes
            )+
        }
    };
}

macro_rules! unary_ops {
    (
        $(#[$meta:meta])*
        fn $trait:ident::$name:ident($slf:ident: &$t:ty) -> $ot:ty $code:block
    ) => {
        op_trait_impl!($trait, $name, 'a, $slf, &'a $t, $ot, $($meta,)* $code);
        op_trait_impl!($trait, $name, $slf, $t, $ot, $($meta,)* { (&$slf).$name() });
    };
}
