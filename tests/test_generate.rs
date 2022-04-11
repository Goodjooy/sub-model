use sub_model::SubModel;

#[derive(SubModel)]
#[sub_model(
    all("SubA"),
    all(
        vis = "",
        name = "SubB",
        extra_field(
            extra_b(ty = "bool", from = "Default::default"),
            extra_c(ty = "String", from = "String::new")
        ),
        extra(derive(Debug))
    ),
    none(
        name = "SubC",
        vis = "pub(self)",
        extra(
            derive(Debug, Default, Clone, PartialEq, Eq, Copy),
            doc = "SubC From MockA"
        )
    )
)]
struct MockA {
    #[sub_model(ignore("SubA"))]
    i: i32,
    #[sub_model(want(
        for = "SubC",
        rename = "pos",
        to_type(ty = "(u16,u16)", by = "u32_t_info")
    ))]
    b: u32,
    #[sub_model(having(for = "SubB", vis = "pub(self)", rename = "bool_c",))]
    c: bool,
}

fn u32_t_info(i: u32) -> (u16, u16) {
    let l = (i >> 16) as u16;
    let r = i as u16;
    (l, r)
}

#[test]
fn test_sub_a() {
    let base = MockA {
        i: 11,
        b: 12,
        c: false,
    };

    let SubA { b, c } = base.into();

    assert_eq!(12, b);
    assert_eq!(false, c);
}

#[test]
fn test_sub_b() {
    let base = MockA {
        i: 11,
        b: 12,
        c: false,
    };

    let b: SubB = base.into();

    println!("{:?}", b);
}

#[test]
fn test_sub_c() {
    let base = MockA {
        i: 11,
        b: 12,
        c: false,
    };

    let c: SubC = base.into();

    print!("{:?}", c.pos);
}
