
use rebound::{Value, Type, ty::CommonTypeInfo};

fn main() {
    let result;
    {
        let a = 1;
        let b = 2;
        let c: &[&i32] = &[&a, &b];
        let val = Value::from(c);

        let fns = val.ty().assoc_fns()
            .into_iter()
            .filter(|assoc| assoc.name() == "first")
            .nth(0)
            .unwrap();

        result = fns.call(Some(val), vec![])
            .unwrap();
    }

    println!("{:p}", result)
}
