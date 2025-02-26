use binaryninja::headless::Session;
use binaryninja::rc::Ref;
use binaryninja::types::{
    EnumerationBuilder, MemberAccess, MemberScope, StructureBuilder, StructureType, Type,
};
use binaryninja_abstract_type::AbstractType;
use rstest::*;

#[fixture]
#[once]
fn session() -> Session {
    Session::new().expect("Failed to initialize session")
}

fn create_struct<F>(f: F) -> Ref<Type>
where
    F: FnOnce(&mut StructureBuilder) -> &mut StructureBuilder,
{
    Type::structure(&f(&mut StructureBuilder::new()).finalize())
}

fn create_enum<F>(width: usize, signed: bool, f: F) -> Ref<Type>
where
    F: FnOnce(&mut EnumerationBuilder) -> &mut EnumerationBuilder,
{
    Type::enumeration(
        &f(&mut EnumerationBuilder::new()).finalize(),
        width.try_into().unwrap(),
        signed,
    )
}

#[rstest]
fn primitive(_session: &Session) {
    assert_eq!(u8::resolve_type(), Type::int(1, false));
    assert_eq!(u16::resolve_type(), Type::int(2, false));
    assert_eq!(u32::resolve_type(), Type::int(4, false));
    assert_eq!(u64::resolve_type(), Type::int(8, false));
    assert_eq!(u128::resolve_type(), Type::int(16, false));

    assert_eq!(i8::resolve_type(), Type::int(1, true));
    assert_eq!(i16::resolve_type(), Type::int(2, true));
    assert_eq!(i32::resolve_type(), Type::int(4, true));
    assert_eq!(i64::resolve_type(), Type::int(8, true));
    assert_eq!(i128::resolve_type(), Type::int(16, true));

    assert_eq!(f32::resolve_type(), Type::float(4));
    assert_eq!(f64::resolve_type(), Type::float(8));
}

#[rstest]
fn basic_struct(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C)]
    struct A {
        first: u8,
        second: u32,
        third: u16,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(1, false),
                "first",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::int(4, false),
                "second",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::int(2, false),
                "third",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
}

#[rstest]
fn packed_struct(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C, packed)]
    struct A {
        first: u8,
        second: u32,
        third: u16,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.packed(true)
                .append(
                    &Type::int(1, false),
                    "first",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
                .append(
                    &Type::int(4, false),
                    "second",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
                .append(
                    &Type::int(2, false),
                    "third",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
        })
    );
}

#[rstest]
fn custom_alignment(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C, align(16))]
    struct A {
        first: u8,
        second: u32,
        third: u16,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.alignment(16)
                .append(
                    &Type::int(1, false),
                    "first",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
                .append(
                    &Type::int(4, false),
                    "second",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
                .append(
                    &Type::int(2, false),
                    "third",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
        })
    );
}

#[rstest]
fn named_field(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C)]
    struct A {
        first: u8,
        #[binja(named)]
        second: B,
    }

    #[derive(AbstractType)]
    #[repr(C)]
    struct B {
        third: u16,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(1, false),
                "first",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::named_type_from_type("B", &B::resolve_type()),
                "second",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
    assert_eq!(
        B::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(2, false),
                "third",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
}

#[rstest]
fn pointer_field(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C)]
    #[binja(pointer_width = 4)]
    struct A {
        first: u8,
        second: *const u32,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(1, false),
                "first",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::pointer_of_width(&Type::int(4, false), 4, false, false, None),
                "second",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
}

#[rstest]
fn nested_pointer_field(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C)]
    struct A {
        first: u8,
        #[binja(named)]
        second: B,
    }

    #[derive(AbstractType)]
    #[repr(C)]
    #[binja(pointer_width = 4)]
    struct B {
        third: u32,
        fourth: *const u16,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(1, false),
                "first",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::named_type_from_type("B", &B::resolve_type()),
                "second",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
    assert_eq!(
        B::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(4, false),
                "third",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::pointer_of_width(&Type::int(2, false), 4, false, false, None),
                "fourth",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
}

#[rstest]
fn named_pointer_field(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C)]
    #[binja(pointer_width = 4)]
    struct A {
        first: u8,
        #[binja(named)]
        second: *const B,
    }

    #[derive(AbstractType)]
    #[repr(C)]
    struct B {
        third: u32,
        fourth: u16,
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(1, false),
                "first",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::pointer_of_width(
                    &Type::named_type_from_type("B", &B::resolve_type()),
                    4,
                    false,
                    false,
                    None,
                ),
                "second",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    );
    assert_eq!(
        B::resolve_type(),
        create_struct(|s| {
            s.append(
                &Type::int(4, false),
                "third",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
            .append(
                &Type::int(2, false),
                "fourth",
                MemberAccess::NoAccess,
                MemberScope::NoScope,
            )
        })
    )
}

#[rstest]
fn union(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(C)]
    union A {
        first: u32,
        second: [u16; 2],
        third: [u8; 4],
    }

    assert_eq!(
        A::resolve_type(),
        create_struct(|s| {
            s.structure_type(StructureType::UnionStructureType)
                .append(
                    &Type::int(4, false),
                    "first",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
                .append(
                    &Type::array(&Type::int(2, false), 2),
                    "second",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
                .append(
                    &Type::array(&Type::int(1, false), 4),
                    "third",
                    MemberAccess::NoAccess,
                    MemberScope::NoScope,
                )
        })
    );
}

#[rstest]
fn enumeration(_session: &Session) {
    #[derive(AbstractType)]
    #[repr(u32)]
    #[allow(dead_code)]
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    assert_eq!(
        Color::resolve_type(),
        create_enum(4, false, |e| {
            e.insert("Red", 0xff0000)
                .insert("Green", 0x00ff00)
                .insert("Blue", 0x0000ff)
        })
    );
}
