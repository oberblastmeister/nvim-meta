#[macro_export]
macro_rules! api {
    {
        $vis:vis async fn $fn_name:ident(
            $nvim:ident: $nvim_ty:ty,
            $($arg:tt: $type:tt),*)
            -> $result_ty:ident<() $(, $err_ty:tt)?>;
    } => {
        $vis async fn $fn_name($nvim: $nvim_ty, $($arg: $type),*) -> $result_ty<$ok_ty $(, $err_ty)?> {
            use nvim_rs::rpc::unpack::TryUnpack;
            use nvim_rs::Value;

            $(let $arg = Value::from($arg);)*
            $nvim.call(stringify!($fn_name), vec![$($arg),*]).await?;
        }
    };

    {
        $vis:vis async fn $fn_name:ident(
            $nvim:ident: $nvim_ty:ty,
            $($arg:tt: $type:ty),*)
            -> $result_ty:ident<$ok_ty:tt $(, $err_ty:tt)?>;
    } => {
        $vis async fn $fn_name($nvim: $nvim_ty, $($arg: $type),*) -> $result_ty<$ok_ty $(, $err_ty)?> {
            use nvim_rs::rpc::unpack::TryUnpack;
            use nvim_rs::Value;

            $(let $arg = Value::from($arg);)*
            let res = $nvim.call(stringify!($fn_name), vec![$($arg),*]).await?.expect("Failed to get value");
            Ok(res.try_unpack().expect("Failed to unpack"))
        }
    };

    {
        $(
            $vis:vis async fn $fn_name:ident($($stuff:tt)*) -> $result_ty:ident<$ok_ty:tt $(, $err_ty:tt)?>;
        )*
    } => {
        $(wrap_api! { $vis async fn $fn_name($($stuff)*) -> $result_ty<$ok_ty $(, $err_ty)?>; })*
    };
}

#[macro_export]
macro_rules! function {
    {
        $vis:vis async fn $fn_name:ident(
            $nvim:ident: $nvim_ty:ty,
            $($arg:tt: $type:ty),*)
            -> $result_ty:ident<() $(, $err_ty:tt)?>;
    } => {
        $vis async fn $fn_name($nvim: $nvim_ty, $($arg: $type),*) -> $result_ty<$ok_ty $(, $err_ty)?> {
            use nvim_rs::rpc::unpack::TryUnpack;
            use nvim_rs::Value;

            $(let $arg = Value::from($arg);)*
            $nvim.call_function(stringify!($fn_name), vec![$($arg),*]).await?;
        }
    };

    {
        $vis:vis async fn $fn_name:ident(
            $nvim:ident: $nvim_ty:ty,
            $($arg:tt: $type:tt),*)
            -> $result_ty:ident<$ok_ty:tt $(, $err_ty:tt)?>;
    } => {
        $vis async fn $fn_name($nvim: $nvim_ty, $($arg: $type),*) -> $result_ty<$ok_ty $(, $err_ty)?> {
            use nvim_rs::rpc::unpack::TryUnpack;
            use nvim_rs::Value;

            $(let $arg = Value::from($arg);)*
            let res = $nvim.call_function(stringify!($fn_name), vec![$($arg),*]).await?;
            Ok(res.try_unpack().expect("Failed to unpack"))
        }
    };

    {
        $(
            $vis:vis async fn $fn_name:ident($($stuff:tt)*) -> $result_ty:ident<$ok_ty:tt $(, $err_ty:tt)?>;
        )*
    } => {
        $(wrap_fn! { $vis async fn $fn_name($($stuff)*) -> $result_ty<$ok_ty $(, $err_ty)?>; })*
    }
}
