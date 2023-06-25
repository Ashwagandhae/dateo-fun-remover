use criterion::{black_box, criterion_group, criterion_main, Criterion};
use datoe_fun_remover::finder::atom::Atom;
use datoe_fun_remover::finder::func::Func;
use datoe_fun_remover::finder::operation::Operation;
use datoe_fun_remover::finder::{create_atom_store, get_solution_with_score};
use strum::IntoEnumIterator;

fn find_bench(c: &mut Criterion) {
    let nums: Vec<f64> = vec![-16., -10., 2., 13., 16.];
    c.bench_function("create_atom_store", |b| {
        b.iter(|| {
            create_atom_store(black_box(&nums));
        })
    });

    let store = create_atom_store(&vec![-16., -10., 2., 13., 16.]);
    c.bench_function("get_solution_with_score", |b| {
        b.iter(|| {
            for i in 0..8 {
                get_solution_with_score(black_box(i), black_box(19.), black_box(&store));
            }
        })
    });
}

fn atom_bench(c: &mut Criterion) {
    let atom_single = Atom::new_express(1., 2., Operation::Add);
    let funcs = &[];
    let distribution = &[];
    c.bench_function("atom_single_eval", |b| {
        b.iter(|| atom_single.eval_with_funcs(funcs, distribution))
    });

    let atom_double = Atom::new_express(
        Atom::new_express(1., 2., Operation::Add),
        Atom::new_express(3., 4., Operation::Add),
        Operation::Add,
    );
    c.bench_function("atom_double_eval", |b| {
        b.iter(|| atom_double.eval_with_funcs(funcs, distribution))
    });

    let atom_none = Atom::new_express(
        Atom::new_express(1., 2., Operation::Add),
        Atom::new_express(4., 0., Operation::Divide),
        Operation::Add,
    );
    c.bench_function("atom_none_eval", |b| {
        b.iter(|| atom_none.eval_with_funcs(funcs, distribution))
    });
}

fn operation_bench(c: &mut Criterion) {
    let left_nums: Vec<f64> = (-50..50).map(|i| i as f64).collect();
    let right_nums: Vec<f64> = (-50..50).map(|i| i as f64).collect();
    for op in Operation::iter() {
        c.bench_function(&format!("operation_{}", op), |b| {
            b.iter(|| {
                for left in &left_nums {
                    for right in &right_nums {
                        op.apply(*left, *right);
                    }
                }
            })
        });
    }
}

fn function_bench(c: &mut Criterion) {
    let nums: Vec<f64> = (-50..50).map(|i| i as f64).collect();
    for func in Func::iter() {
        c.bench_function(&format!("function_{}", func), |b| {
            b.iter(|| {
                for num in &nums {
                    func.apply(*num);
                }
            })
        });
    }
}

criterion_group!(
  name = find;
  config = Criterion::default().sample_size(10000);
  targets = find_bench
);
criterion_group!(atom, atom_bench);
criterion_group!(operation, operation_bench);
criterion_group!(function, function_bench);

criterion_main!(find, atom, operation, function);
