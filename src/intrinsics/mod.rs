#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;

use runtime::{Flow, Runtime, Side};
use ast;
use prelude::{Lt, Prelude, Dfn};

use FnIndex;
use Error;
use Module;
use Variable;
use Type;
use TINVOTS;

mod io;
mod meta;
mod data;
mod lifetimechk;

const X: usize = 0;
const Y: usize = 1;
const Z: usize = 2;
const W: usize = 3;
const WHY: usize = 4;
const WHERE: usize = 5;
const EXPLAIN_WHY: usize = 6;
const EXPLAIN_WHERE: usize = 7;
const PRINTLN: usize = 8;
const PRINT: usize = 9;
const CLONE: usize = 10;
const DEBUG: usize = 11;
const BACKTRACE: usize = 12;
const SLEEP: usize = 13;
const RANDOM: usize = 14;
const HEAD: usize = 15;
const TAIL: usize = 16;
const IS_EMPTY: usize = 17;
const READ_NUMBER: usize = 18;
const READ_LINE: usize = 19;
const LEN: usize = 20;
const PUSH_REF: usize = 21;
const PUSH: usize = 22;
const POP: usize = 23;
const REVERSE: usize = 24;
const CLEAR: usize = 25;
const SWAP: usize = 26;
const TRIM: usize = 27;
const TRIM_LEFT: usize = 28;
const TRIM_RIGHT: usize = 29;
const STR: usize = 30;
const JSON_STRING: usize = 31;
const STR__COLOR: usize = 32;
const SRGB_TO_LINEAR__COLOR: usize = 33;
const LINEAR_TO_SRGB__COLOR: usize = 34;
const TYPEOF: usize = 35;
const ROUND: usize = 36;
const ABS: usize = 37;
const FLOOR: usize = 38;
const CEIL: usize = 39;
const SQRT: usize = 40;
const SIN: usize = 41;
const ASIN: usize = 42;
const COS: usize = 43;
const ACOS: usize = 44;
const TAN: usize = 45;
const ATAN: usize = 46;
const EXP: usize = 47;
const LN: usize = 48;
const LOG2: usize = 49;
const LOG10: usize = 50;
const LOAD: usize = 51;
const LOAD__SOURCE_IMPORTS: usize = 52;
const CALL: usize = 53;
const CALL_RET: usize = 54;
const FUNCTIONS: usize = 55;
const NONE: usize = 56;
const SOME: usize = 57;
const UNWRAP: usize = 58;
const UNWRAP_ERR: usize = 59;
const OK: usize = 60;
const ERR: usize = 61;
const IS_ERR: usize = 62;
const IS_OK: usize = 63;
const MIN: usize = 64;
const MAX: usize = 65;
const S: usize = 66;
const DIR__ANGLE: usize = 67;
const LOAD__META_FILE: usize = 68;
const LOAD__META_URL: usize = 69;
const DOWNLOAD__URL_FILE: usize = 70;
const SAVE__STRING_FILE: usize = 71;
const LOAD_STRING__FILE: usize = 72;
const JOIN__THREAD: usize = 73;
const SAVE__DATA_FILE: usize = 74;
const JSON_FROM_META_DATA: usize = 75;
const HAS: usize = 76;
const CHARS: usize = 77;
const NOW: usize = 78;
const IS_NAN: usize = 79;
const ATAN2: usize = 80;
const UNWRAP_OR: usize = 81;
const TIP: usize = 82;
const NECK: usize = 83;
const LOAD_DATA__FILE: usize = 84;

const TABLE: &'static [(usize, fn(
        &mut Runtime,
        &ast::Call,
        &Module,
        usize,
        usize,
        usize
    ) -> Result<Option<Variable>, String>)]
= &[
    (X, x),
    (Y, y),
    (Z, z),
    (W, w),
    (WHY, why),
    (WHERE, _where),
    (EXPLAIN_WHY, explain_why),
    (EXPLAIN_WHERE, explain_where),
    (PRINTLN, println),
    (PRINT, print),
    (CLONE, clone),
    (DEBUG, debug),
    (BACKTRACE, backtrace),
    (SLEEP, sleep),
    (RANDOM, random),
    (HEAD, head),
    (TAIL, tail),
    (IS_EMPTY, is_empty),
    (READ_NUMBER, read_number),
    (READ_LINE, read_line),
    (LEN, len),
    (PUSH_REF, push_ref),
    (PUSH, push),
    (POP, pop),
    (REVERSE, reverse),
    (CLEAR, clear),
    (SWAP, swap),
    (TRIM, trim),
    (TRIM_LEFT, trim_left),
    (TRIM_RIGHT, trim_right),
    (STR, _str),
    (JSON_STRING, json_string),
    (STR__COLOR, str__color),
    (SRGB_TO_LINEAR__COLOR, srgb_to_linear__color),
    (LINEAR_TO_SRGB__COLOR, linear_to_srgb__color),
    (TYPEOF, _typeof),
    (ROUND, round),
    (ABS, abs),
    (FLOOR, floor),
    (CEIL, ceil),
    (SQRT, sqrt),
    (SIN, sin),
    (ASIN, asin),
    (COS, cos),
    (ACOS, acos),
    (TAN, tan),
    (ATAN, atan),
    (EXP, exp),
    (LN, ln),
    (LOG2, log2),
    (LOG10, log10),
    (LOAD, load),
    (LOAD__SOURCE_IMPORTS, load__source_imports),
    (CALL, _call),
    (CALL_RET, call_ret),
    (FUNCTIONS, functions),
    (NONE, none),
    (SOME, some),
    (UNWRAP, unwrap),
    (UNWRAP_ERR, unwrap_err),
    (OK, ok),
    (ERR, err),
    (IS_ERR, is_err),
    (IS_OK, is_ok),
    (MIN, min),
    (MAX, max),
    (S, s),
    (DIR__ANGLE, dir__angle),
    (LOAD__META_FILE, load__meta_file),
    (LOAD__META_URL, load__meta_url),
    (DOWNLOAD__URL_FILE, download__url_file),
    (SAVE__STRING_FILE, save__string_file),
    (LOAD_STRING__FILE, load_string__file),
    (JOIN__THREAD, join__thread),
    (SAVE__DATA_FILE, save__data_file),
    (JSON_FROM_META_DATA, json_from_meta_data),
    (HAS, has),
    (CHARS, chars),
    (NOW, now),
    (IS_NAN, is_nan),
    (ATAN2, atan2),
    (UNWRAP_OR, unwrap_or),
    (TIP, tip),
    (NECK, neck),
    (LOAD_DATA__FILE, load_data__file),
];

pub fn standard(f: &mut Prelude) {
    let sarg = |f: &mut Prelude, name: &str, index: usize, ty: Type, ret: Type| {
        f.intrinsic(Arc::new(name.into()), index, Dfn {
            lts: vec![Lt::Default],
            tys: vec![ty],
            ret: ret
        });
    };

    sarg(f, "x", X, Type::Vec4, Type::F64);
    sarg(f, "y", Y, Type::Vec4, Type::F64);
    sarg(f, "z", Z, Type::Vec4, Type::F64);
    sarg(f, "w", W, Type::Vec4, Type::F64);
    f.intrinsic(Arc::new("why".into()), WHY, Dfn {
        lts: vec![Lt::Default],
        tys: vec![Type::Secret(Box::new(Type::Bool))],
        ret: Type::array()
    });
    f.intrinsic(Arc::new("where".into()), WHERE, Dfn {
        lts: vec![Lt::Default],
        tys: vec![Type::Secret(Box::new(Type::F64))],
        ret: Type::array()
    });
    f.intrinsic(Arc::new("explain_why".into()), EXPLAIN_WHY, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Bool, Type::Any],
        ret: Type::Secret(Box::new(Type::Bool))
    });
    f.intrinsic(Arc::new("explain_where".into()), EXPLAIN_WHERE, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::F64, Type::Any],
        ret: Type::Secret(Box::new(Type::F64))
    });
    sarg(f, "println", PRINTLN, Type::Any, Type::Void);
    sarg(f, "print", PRINT, Type::Any, Type::Void);
    sarg(f, "clone", CLONE, Type::Any, Type::Any);
    f.intrinsic(Arc::new("debug".into()), DEBUG, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Void
    });
    f.intrinsic(Arc::new("backtrace".into()), BACKTRACE, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Void
    });
    sarg(f, "sleep", SLEEP, Type::F64, Type::Void);
    f.intrinsic(Arc::new("random".into()), RANDOM, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::F64
    });
    sarg(f, "head", HEAD, Type::Link, Type::Any);
    sarg(f, "tail", TAIL, Type::Link, Type::Link);
    sarg(f, "is_empty", IS_EMPTY, Type::Link, Type::Bool);
    sarg(f, "read_number", READ_NUMBER, Type::Text, Type::F64);
    f.intrinsic(Arc::new("read_line".into()), READ_LINE, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Text
    });
    sarg(f, "len", LEN, Type::array(), Type::F64);
    f.intrinsic(Arc::new("push_ref(mut,_)".into()), PUSH_REF, Dfn {
        lts: vec![Lt::Default, Lt::Arg(0)],
        tys: vec![Type::array(), Type::Any],
        ret: Type::Void
    });
    f.intrinsic(Arc::new("push(mut,_)".into()), PUSH, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::array(), Type::Any],
        ret: Type::Void
    });
    f.intrinsic(Arc::new("pop(mut)".into()), POP, Dfn {
        lts: vec![Lt::Return],
        tys: vec![Type::array()],
        ret: Type::Any
    });
    sarg(f, "reverse(mut)", REVERSE, Type::array(), Type::Void);
    sarg(f, "clear(mut)", CLEAR, Type::array(), Type::Void);
    f.intrinsic(Arc::new("swap(mut,_,_)".into()), SWAP, Dfn {
        lts: vec![Lt::Default; 3],
        tys: vec![Type::array(), Type::F64, Type::F64],
        ret: Type::Void
    });
    sarg(f, "trim", TRIM, Type::Text, Type::Text);
    sarg(f, "trim_left", TRIM_LEFT, Type::Text, Type::Text);
    sarg(f, "trim_right", TRIM_RIGHT, Type::Text, Type::Text);
    sarg(f, "str", STR, Type::Any, Type::Text);
    sarg(f, "json_string", JSON_STRING, Type::Text, Type::Text);
    sarg(f, "str__color", STR__COLOR, Type::Vec4, Type::Text);
    sarg(f, "srgb_to_linear__color", SRGB_TO_LINEAR__COLOR, Type::Vec4, Type::Vec4);
    sarg(f, "linear_to_srgb__color", LINEAR_TO_SRGB__COLOR, Type::Vec4, Type::Vec4);
    sarg(f, "typeof", TYPEOF, Type::Any, Type::Text);
    sarg(f, "round", ROUND, Type::F64, Type::F64);
    sarg(f, "abs", ABS, Type::F64, Type::F64);
    sarg(f, "floor", FLOOR, Type::F64, Type::F64);
    sarg(f, "ceil", CEIL, Type::F64, Type::F64);
    sarg(f, "sqrt", SQRT, Type::F64, Type::F64);
    sarg(f, "sin", SIN, Type::F64, Type::F64);
    sarg(f, "asin", ASIN, Type::F64, Type::F64);
    sarg(f, "cos", COS, Type::F64, Type::F64);
    sarg(f, "acos", ACOS, Type::F64, Type::F64);
    sarg(f, "tan", TAN, Type::F64, Type::F64);
    sarg(f, "atan", ATAN, Type::F64, Type::F64);
    sarg(f, "exp", EXP, Type::F64, Type::F64);
    sarg(f, "ln", LN, Type::F64, Type::F64);
    sarg(f, "log2", LOG2, Type::F64, Type::F64);
    sarg(f, "log10", LOG10, Type::F64, Type::F64);
    sarg(f, "load", LOAD, Type::Text, Type::result());
    f.intrinsic(Arc::new("load__source_imports".into()), LOAD__SOURCE_IMPORTS, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Text, Type::array()],
        ret: Type::result()
    });
    f.intrinsic(Arc::new("call".into()), CALL, Dfn {
        lts: vec![Lt::Default; 3],
        tys: vec![Type::Any, Type::Text, Type::array()],
        ret: Type::Void
    });
    f.intrinsic(Arc::new("call_ret".into()), CALL_RET, Dfn {
        lts: vec![Lt::Default; 3],
        tys: vec![Type::Any, Type::Text, Type::array()],
        ret: Type::Any
    });
    f.intrinsic(Arc::new("functions".into()), FUNCTIONS, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::Any
    });
    f.intrinsic(Arc::new("none".into()), NONE, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::option()
    });
    sarg(f, "some", SOME, Type::Any, Type::option());
    sarg(f, "unwrap", UNWRAP, Type::Any, Type::Any);
    sarg(f, "unwrap_err", UNWRAP_ERR, Type::Any, Type::Any);
    sarg(f, "ok", OK, Type::Any, Type::result());
    sarg(f, "err", ERR, Type::Any, Type::result());
    sarg(f, "is_err", IS_ERR, Type::result(), Type::Bool);
    sarg(f, "is_ok", IS_OK, Type::result(), Type::Bool);
    sarg(f, "min", MIN, Type::Array(Box::new(Type::F64)), Type::F64);
    sarg(f, "max", MAX, Type::Array(Box::new(Type::F64)), Type::F64);
    f.intrinsic(Arc::new("s".into()), S, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Vec4, Type::F64],
        ret: Type::F64
    });
    sarg(f, "dir__angle", DIR__ANGLE, Type::F64, Type::Vec4);
    f.intrinsic(Arc::new("load__meta_file".into()), LOAD__META_FILE, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Text; 2],
        ret: Type::Result(Box::new(Type::Array(Box::new(Type::array()))))
    });
    f.intrinsic(Arc::new("load__meta_url".into()), LOAD__META_URL, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Text; 2],
        ret: Type::Result(Box::new(Type::array()))
    });
    f.intrinsic(Arc::new("download__url_file".into()), DOWNLOAD__URL_FILE, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Text; 2],
        ret: Type::Result(Box::new(Type::Text))
    });
    f.intrinsic(Arc::new("save__string_file".into()), SAVE__STRING_FILE, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Text; 2],
        ret: Type::Result(Box::new(Type::Text))
    });
    sarg(f, "load_string__file", LOAD_STRING__FILE, Type::Text, Type::Result(Box::new(Type::Text)));
    sarg(f, "join__thread", JOIN__THREAD, Type::thread(), Type::Result(Box::new(Type::Any)));
    f.intrinsic(Arc::new("save__data_file".into()), SAVE__DATA_FILE, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Any, Type::Text],
        ret: Type::Result(Box::new(Type::Text))
    });
    sarg(f, "json_from_meta_data", JSON_FROM_META_DATA, Type::Array(Box::new(Type::array())), Type::Text);
    f.intrinsic(Arc::new("has".into()), HAS, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Object, Type::Text],
        ret: Type::Bool
    });
    sarg(f, "chars", CHARS, Type::Text, Type::Array(Box::new(Type::Text)));
    f.intrinsic(Arc::new("now".into()), NOW, Dfn {
        lts: vec![],
        tys: vec![],
        ret: Type::F64
    });
    sarg(f, "is_nan", IS_NAN, Type::F64, Type::Bool);
    f.intrinsic(Arc::new("atan2".into()), ATAN2, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::F64; 2],
        ret: Type::F64
    });
    f.intrinsic(Arc::new("unwrap_or".into()), UNWRAP_OR, Dfn {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::Any, Type::Any],
        ret: Type::Any
    });
    sarg(f, "tip", TIP, Type::Link, Type::Option(Box::new(Type::Any)));
    sarg(f, "neck", NECK, Type::Link, Type::Link);
    sarg(f, "load_data__file", LOAD_DATA__FILE, Type::Text, Type::Result(Box::new(Type::Any)));
}

pub fn call_standard(
    rt: &mut Runtime,
    index: usize,
    call: &ast::Call,
    module: &Module
) -> Result<(Option<Variable>, Flow), String> {
    let st = rt.stack.len();
    let lc = rt.local_stack.len();
    let cu = rt.current_stack.len();
    for arg in &call.args {
        match try!(rt.expression(arg, Side::Right, module)) {
            (x, Flow::Return) => { return Ok((x, Flow::Return)); }
            (Some(v), Flow::Continue) => rt.stack.push(v),
            _ => return Err(module.error(arg.source_range(),
                    &format!("{}\nExpected something. \
                    Expression did not return a value.",
                    rt.stack_trace()), rt))
        };
    }
    let (ind, f) = TABLE[index];
    debug_assert!(ind == index);
    let expect = try!((f)(rt, call, module, st, lc, cu));
    Ok((expect, Flow::Continue))
}

fn x(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Vec4(ref vec4) => Variable::f64(vec4[0] as f64),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    }))
}

fn y(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Vec4(ref vec4) => Variable::f64(vec4[1] as f64),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    }))
}

fn z(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Vec4(ref vec4) => Variable::f64(vec4[2] as f64),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    }))
}

fn w(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Vec4(ref vec4) => Variable::f64(vec4[3] as f64),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    }))
}

fn s(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let ind = rt.stack.pop().expect(TINVOTS);
    let ind = match rt.resolve(&ind) {
        &Variable::F64(val, _) => val,
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "number"), rt))
    };
    let v = rt.stack.pop().expect(TINVOTS);
    let s = match rt.resolve(&v) {
        &Variable::Vec4(ref v) => {
            match v.get(ind as usize) {
                Some(&s) => s as f64,
                None => {
                    return Err(module.error(call.source_range,
                        &format!("{}\nIndex out of bounds `{}`",
                            rt.stack_trace(), ind), rt))
                }
            }
        }
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "vec4"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::f64(s)))
}

fn clone(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(rt.resolve(&v).deep_clone(&rt.stack)))
}

fn why(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Array(Arc::new(match rt.resolve(&v) {
        &Variable::Bool(true, Some(ref sec)) => {
            let mut sec = (**sec).clone();
            sec.reverse();
            sec
        }
        &Variable::Bool(true, None) => {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nThis does not make sense, perhaps an array is empty?",
                    rt.stack_trace()), rt))
        }
        &Variable::Bool(false, _) => {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nMust be `true` to have meaning, try add or remove `!`",
                    rt.stack_trace()), rt))
        }
        x => return Err(module.error(call.args[0].source_range(),
            &rt.expected(x, "bool"), rt))
    }));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn _where(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Array(Arc::new(match rt.resolve(&v) {
        &Variable::F64(val, Some(ref sec)) => {
            if val.is_nan() {
                return Err(module.error(call.args[0].source_range(),
                    &format!("{}\nExpected number, found `NaN`",
                        rt.stack_trace()), rt))
            } else {
                let mut sec = (**sec).clone();
                sec.reverse();
                sec
            }
        }
        &Variable::F64(_, None) => {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nThis does not make sense, perhaps an array is empty?",
                    rt.stack_trace()), rt))
        }
        x => return Err(module.error(call.args[0].source_range(),
            &rt.expected(x, "f64"), rt))
    }));
    Ok(Some(v))
}

fn explain_why(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let why = rt.stack.pop().expect(TINVOTS);
    let val = rt.stack.pop().expect(TINVOTS);
    let (val, why) = match rt.resolve(&val) {
        &Variable::Bool(val, ref sec) => (val,
            match sec {
                &None => Box::new(vec![why.deep_clone(&rt.stack)]),
                &Some(ref sec) => {
                    let mut sec = sec.clone();
                    sec.push(why.deep_clone(&rt.stack));
                    sec
                }
            }
        ),
        x => return Err(module.error(call.args[0].source_range(),
            &rt.expected(x, "bool"), rt))
    };
    Ok(Some(Variable::Bool(val, Some(why))))
}

fn explain_where(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let wh = rt.stack.pop().expect(TINVOTS);
    let val = rt.stack.pop().expect(TINVOTS);
    let (val, wh) = match rt.resolve(&val) {
        &Variable::F64(val, ref sec) => (val,
            match sec {
                &None => Box::new(vec![wh.deep_clone(&rt.stack)]),
                &Some(ref sec) => {
                    let mut sec = sec.clone();
                    sec.push(wh.deep_clone(&rt.stack));
                    sec
                }
            }
        ),
        x => return Err(module.error(call.args[0].source_range(),
            &rt.expected(x, "bool"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::F64(val, Some(wh))))
}

fn println(
    rt: &mut Runtime,
    call: &ast::Call,
    _module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use write::{print_variable, EscapeString};

    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let x = rt.stack.pop().expect(TINVOTS);
    print_variable(rt, &x, EscapeString::None);
    println!("");
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn print(
    rt: &mut Runtime,
    call: &ast::Call,
    _module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use write::{print_variable, EscapeString};

    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let x = rt.stack.pop().expect(TINVOTS);
    print_variable(rt, &x, EscapeString::None);
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn sqrt(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.sqrt())
}

fn sin(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.sin())
}

fn asin(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.asin())
}

fn cos(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.cos())
}

fn acos(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.acos())
}

fn tan(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.tan())
}

fn atan(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.atan())
}

fn atan2(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let x = rt.stack.pop().expect(TINVOTS);
    let x = match rt.resolve(&x) {
        &Variable::F64(b, _) => b,
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    };
    let y = rt.stack.pop().expect(TINVOTS);
    let y = match rt.resolve(&y) {
        &Variable::F64(b, _) => b,
        y => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(y, "number"), rt))
    };
    Ok(Some(Variable::f64(y.atan2(x))))
}

fn exp(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.exp())
}

fn ln(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.ln())
}

fn log2(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.log2())
}

fn log10(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.log10())
}

fn round(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.round())
}

fn abs(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.abs())
}

fn floor(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.floor())
}

fn ceil(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    rt.unary_f64(call, module, |a| a.ceil())
}

fn sleep(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    use std::thread::sleep;
    use std::time::Duration;

    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::F64(b, _) => b,
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    };
    let secs = v as u64;
    let nanos = (v.fract() * 1.0e9) as u32;
    sleep(Duration::new(secs, nanos));
    Ok(None)
}

fn head(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Option(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.head(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "link"), rt))
    });
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn tip(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Option(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.tip(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "link"), rt))
    });
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn tail(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Link(Box::new(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.tail(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "link"), rt))
    }));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn neck(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Link(Box::new(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.neck(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "link"), rt))
    }));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn is_empty(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(Variable::bool(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.is_empty(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "link"), rt))
    })))
}

fn random(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    Ok(Some(Variable::f64(rt.rng.gen())))
}

fn len(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = match rt.stack.pop() {
        Some(v) => v,
        None => panic!(TINVOTS)
    };

    let v = {
        let arr = match rt.resolve(&v) {
            &Variable::Array(ref arr) => arr,
            x => return Err(module.error(call.args[0].source_range(),
                            &rt.expected(x, "array"), rt))
        };
        Variable::f64(arr.len() as f64)
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn push_ref(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let item = rt.stack.pop().expect(TINVOTS);
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).push(item);
            true
        } else {
            false
        };
        if !ok {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected reference to array",
                    rt.stack_trace()), rt));
        }
    } else {
        return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected reference to array",
                rt.stack_trace()), rt));
    }
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn push(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let item = rt.stack.pop().expect(TINVOTS);
    let item = rt.resolve(&item).deep_clone(&rt.stack);
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).push(item);
            true
        } else {
            false
        };
        if !ok {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected reference to array",
                    rt.stack_trace()), rt));
        }
    } else {
        return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected reference to array",
                rt.stack_trace()), rt));
    }
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn pop(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let arr = rt.stack.pop().expect(TINVOTS);
    let mut v: Option<Variable> = None;
    if let Variable::Ref(ind) = arr {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            v = Arc::make_mut(arr).pop();
            true
        } else {
            false
        };
        if !ok {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected reference to array",
                    rt.stack_trace()), rt));
        }
    } else {
        return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected reference to array",
                rt.stack_trace()), rt));
    }
    let v = match v {
        None => return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected non-empty array",
                rt.stack_trace()), rt)),
        Some(val) => val
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn reverse(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).reverse();
            true
        } else {
            false
        };
        if !ok {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected reference to array",
                    rt.stack_trace()), rt));
        }
    } else {
        return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected reference to array",
                rt.stack_trace()), rt));
    }
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn clear(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).clear();
            true
        } else {
            false
        };
        if !ok {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected reference to array",
                    rt.stack_trace()), rt));
        }
    } else {
        return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected reference to array",
                rt.stack_trace()), rt));
    }
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn swap(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let j = rt.stack.pop().expect(TINVOTS);
    let i = rt.stack.pop().expect(TINVOTS);
    let j = match rt.resolve(&j) {
        &Variable::F64(val, _) => val,
        x => return Err(module.error(call.args[2].source_range(),
            &rt.expected(x, "number"), rt))
    };
    let i = match rt.resolve(&i) {
        &Variable::F64(val, _) => val,
        x => return Err(module.error(call.args[1].source_range(),
            &rt.expected(x, "number"), rt))
    };
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).swap(i as usize, j as usize);
            true
        } else {
            false
        };
        if !ok {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected reference to array",
                    rt.stack_trace()), rt));
        }
    } else {
        return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected reference to array",
                rt.stack_trace()), rt));
    }
    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn read_line(
    rt: &mut Runtime,
    call: &ast::Call,
    _module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use std::io::{self, Write};

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let mut input = String::new();
    io::stdout().flush().unwrap();
    let error = match io::stdin().read_line(&mut input) {
        Ok(_) => None,
        Err(error) => Some(error)
    };
    let v = if let Some(error) = error {
        // TODO: Return error instead.
        Variable::RustObject(
            Arc::new(Mutex::new(error)))
    } else {
        Variable::Text(Arc::new(input))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn read_number(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use std::io::{self, Write};

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let err = rt.stack.pop().expect(TINVOTS);
    let err = match rt.resolve(&err) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "text"), rt))
    };
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut rv: Option<Variable> = None;
    loop {
        input.clear();
        stdout.flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(_) => {}
            Err(error) => {
                // TODO: Return error instead.
                rt.stack.push(Variable::RustObject(
                    Arc::new(Mutex::new(error))));
                break;
            }
        };
        match input.trim().parse::<f64>() {
            Ok(v) => {
                rv = Some(Variable::f64(v));
                break;
            }
            Err(_) => {
                println!("{}", err);
            }
        }
    }
    rt.pop_fn(call.name.clone());
    Ok(rv)
}

fn trim(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "text"), rt))
    };
    let v = Variable::Text(Arc::new(v.trim().into()));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn trim_left(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "text"), rt))
    };
    let v = Variable::Text(Arc::new(v.trim_left().into()));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn trim_right(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let mut v = match rt.resolve(&v) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "text"), rt))
    };
    {
        let w = Arc::make_mut(&mut v);
        while let Some(ch) = w.pop() {
            if !ch.is_whitespace() { w.push(ch); break; }
        }
    }
    Ok(Some(Variable::Text(v)))
}

fn _str(
    rt: &mut Runtime,
    call: &ast::Call,
    _module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use write::{write_variable, EscapeString};

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let mut buf: Vec<u8> = vec![];
    write_variable(&mut buf, rt, rt.resolve(&v), EscapeString::None, 0).unwrap();
    let v = Variable::Text(Arc::new(String::from_utf8(buf).unwrap()));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn json_string(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    use write::{write_variable, EscapeString};

    let v = rt.stack.pop().expect(TINVOTS);
    let mut buf: Vec<u8> = vec![];
    write_variable(&mut buf, rt, rt.resolve(&v), EscapeString::Json, 0).unwrap();
    Ok(Some(Variable::Text(Arc::new(String::from_utf8(buf).unwrap()))))
}

fn str__color(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Vec4(val) => val,
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "vec4"), rt))
    };
    let mut buf: Vec<u8> = vec![];
    let clamp = |x| {
        if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
    };
    let r = (clamp(v[0]) * 255.0) as usize;
    let g = (clamp(v[1]) * 255.0) as usize;
    let b = (clamp(v[2]) * 255.0) as usize;
    let a = (clamp(v[3]) * 255.0) as usize;
    let map = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
               'a', 'b', 'c', 'd', 'e', 'f'];
    let (r1, r2) = (r >> 4, r & 0xf);
    let (g1, g2) = (g >> 4, g & 0xf);
    let (b1, b2) = (b >> 4, b & 0xf);
    let (a1, a2) = (a >> 4, a & 0xf);
    buf.push('#' as u8);
    buf.push(map[r1] as u8); buf.push(map[r2] as u8);
    buf.push(map[g1] as u8); buf.push(map[g2] as u8);
    buf.push(map[b1] as u8); buf.push(map[b2] as u8);
    if a != 255 {
        buf.push(map[a1] as u8); buf.push(map[a2] as u8);
    }
    Ok(Some(Variable::Text(Arc::new(String::from_utf8(buf).unwrap()))))
}

fn srgb_to_linear__color(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Vec4(val) => val,
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "vec4"), rt))
    };
    let to_linear = |f: f32| {
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    };
    Ok(Some(Variable::Vec4(
        [to_linear(v[0]), to_linear(v[1]), to_linear(v[2]), v[3]]
    )))
}

fn linear_to_srgb__color(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Vec4(val) => val,
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "vec4"), rt))
    };
    let to_srgb = |f: f32| {
        if f <= 0.0031308 {
            f * 12.92
        } else {
            1.055 * f.powf(1.0 / 2.4) - 0.055
        }
    };
    Ok(Some(Variable::Vec4(
        [to_srgb(v[0]), to_srgb(v[1]), to_srgb(v[2]), v[3]]
    )))
}

fn _typeof(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Text(_) => rt.text_type.clone(),
        &Variable::F64(_, _) => rt.f64_type.clone(),
        &Variable::Vec4(_) => rt.vec4_type.clone(),
        &Variable::Return => rt.return_type.clone(),
        &Variable::Bool(_, _) => rt.bool_type.clone(),
        &Variable::Object(_) => rt.object_type.clone(),
        &Variable::Array(_) => rt.array_type.clone(),
        &Variable::Link(_) => rt.link_type.clone(),
        &Variable::Ref(_) => rt.ref_type.clone(),
        &Variable::UnsafeRef(_) => rt.unsafe_ref_type.clone(),
        &Variable::RustObject(_) => rt.rust_object_type.clone(),
        &Variable::Option(_) => rt.option_type.clone(),
        &Variable::Result(_) => rt.result_type.clone(),
        &Variable::Thread(_) => rt.thread_type.clone(),
        &Variable::Closure(_, _) => rt.closure_type.clone(),
    }))
}

fn debug(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    println!("Stack {:#?}", rt.stack);
    println!("Locals {:#?}", rt.local_stack);
    println!("Currents {:#?}", rt.current_stack);
    Ok(None)
}

fn backtrace(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    println!("{:#?}", rt.call_stack);
    Ok(None)
}

fn load(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use load;

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Text(ref text) => {
            let mut m = Module::new_intrinsics(module.intrinsics.clone());
            for f in &module.ext_prelude {
                m.add(f.name.clone(), f.f, f.p.clone());
            }
            if let Err(err) = load(text, &mut m) {
                Variable::Result(Err(Box::new(Error {
                    message: Variable::Text(Arc::new(
                        format!("{}\n{}\n{}", rt.stack_trace(), err,
                            module.error(call.args[0].source_range(),
                            "When attempting to load module:", rt)))),
                    trace: vec![]
                })))
            } else {
                Variable::Result(Ok(Box::new(
                    Variable::RustObject(Arc::new(Mutex::new(m))))))
            }
        }
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "string"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn load__source_imports(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use load;

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let modules = rt.stack.pop().expect(TINVOTS);
    let source = rt.stack.pop().expect(TINVOTS);
    let mut new_module = Module::new_intrinsics(module.intrinsics.clone());
    for f in &module.ext_prelude {
        new_module.add(f.name.clone(), f.f, f.p.clone());
    }
    match rt.resolve(&modules) {
        &Variable::Array(ref array) => {
            for it in &**array {
                match rt.resolve(it) {
                    &Variable::RustObject(ref obj) => {
                        match obj.lock().unwrap().downcast_ref::<Module>() {
                            Some(m) => {
                                // Add external functions from imports.
                                for f in &m.ext_prelude {
                                    let has_external = new_module.ext_prelude.iter()
                                        .any(|a| a.name == f.name);
                                    if !has_external {
                                        new_module.add(f.name.clone(), f.f, f.p.clone());
                                    }
                                }
                                // Register loaded functions from imports.
                                for f in &m.functions {
                                    new_module.register(f.clone())
                                }
                            }
                            None => return Err(module.error(
                                call.args[1].source_range(),
                                &format!("{}\nExpected `Module`",
                                    rt.stack_trace()), rt))
                        }
                    }
                    x => return Err(module.error(
                        call.args[1].source_range(),
                        &rt.expected(x, "Module"), rt))
                }
            }
        }
        x => return Err(module.error(call.args[1].source_range(),
                &rt.expected(x, "[Module]"), rt))
    }
    let v = match rt.resolve(&source) {
        &Variable::Text(ref text) => {
            if let Err(err) = load(text, &mut new_module) {
                Variable::Result(Err(Box::new(Error {
                    message: Variable::Text(Arc::new(
                        format!("{}\n{}\n{}", rt.stack_trace(), err,
                            module.error(call.args[0].source_range(),
                            "When attempting to load module:", rt)))),
                    trace: vec![]
                })))
            } else {
                Variable::Result(Ok(Box::new(
                    Variable::RustObject(Arc::new(
                        Mutex::new(new_module))))))
            }
        }
        x => return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "[Module]"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn _call(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    // Use the source from calling function.
    let source = module.functions[rt.call_stack.last().unwrap().index].source.clone();
    rt.push_fn(call.name.clone(), 0, None, st, lc, cu);
    let args = rt.stack.pop().expect(TINVOTS);
    let fn_name = rt.stack.pop().expect(TINVOTS);
    let call_module = rt.stack.pop().expect(TINVOTS);
    let fn_name = match rt.resolve(&fn_name) {
        &Variable::Text(ref text) => text.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "text"), rt))
    };
    let args = match rt.resolve(&args) {
        &Variable::Array(ref arr) => arr.clone(),
        x => return Err(module.error(call.args[2].source_range(),
                        &rt.expected(x, "array"), rt))
    };
    let obj = match rt.resolve(&call_module) {
        &Variable::RustObject(ref obj) => obj.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "Module"), rt))
    };

    match obj.lock().unwrap()
        .downcast_ref::<Module>() {
        Some(m) => {
            use std::cell::Cell;

            let f_index = m.find_function(&fn_name, 0);
            match f_index {
                FnIndex::Loaded(f_index) => {
                    let f = &m.functions[f_index as usize];
                    if f.args.len() != args.len() {
                        return Err(module.error(
                            call.args[2].source_range(),
                            &format!(
                                "{}\nExpected `{}` arguments, found `{}`",
                                rt.stack_trace(),
                                f.args.len(), args.len()), rt))
                    }
                    try!(lifetimechk::check(f, &args).map_err(|err|
                        module.error(call.args[2].source_range(),
                        &format!("{}\n{}", err, rt.stack_trace()), rt)));
                }
                FnIndex::Intrinsic(_) | FnIndex::None | FnIndex::External(_) =>
                    return Err(module.error(
                            call.args[1].source_range(),
                            &format!(
                                "{}\nCould not find function `{}`",
                                rt.stack_trace(),
                                fn_name), rt))
            }
            let call = ast::Call {
                name: fn_name.clone(),
                f_index: Cell::new(f_index),
                args: args.iter().map(|arg|
                    ast::Expression::Variable(
                        call.source_range, arg.clone())).collect(),
                custom_source: Some(source),
                source_range: call.source_range,
            };

            try!(rt.call(&call, &m));
        }
        None => return Err(module.error(call.args[0].source_range(),
                    &format!("{}\nExpected `Module`",
                        rt.stack_trace()), rt))
    }

    rt.pop_fn(call.name.clone());
    Ok(None)
}

fn call_ret(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    // Use the source from calling function.
    let source = module.functions[rt.call_stack.last().unwrap().index].source.clone();
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let args = rt.stack.pop().expect(TINVOTS);
    let fn_name = rt.stack.pop().expect(TINVOTS);
    let call_module = rt.stack.pop().expect(TINVOTS);
    let fn_name = match rt.resolve(&fn_name) {
        &Variable::Text(ref text) => text.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "text"), rt))
    };
    let args = match rt.resolve(&args) {
        &Variable::Array(ref arr) => arr.clone(),
        x => return Err(module.error(call.args[2].source_range(),
                        &rt.expected(x, "array"), rt))
    };
    let obj = match rt.resolve(&call_module) {
        &Variable::RustObject(ref obj) => obj.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "Module"), rt))
    };

    let v = match obj.lock().unwrap()
        .downcast_ref::<Module>() {
        Some(m) => {
            use std::cell::Cell;

            let f_index = m.find_function(&fn_name, 0);
            match f_index {
                FnIndex::Loaded(f_index) => {
                    let f = &m.functions[f_index as usize];
                    if f.args.len() != args.len() {
                        return Err(module.error(
                            call.args[2].source_range(),
                            &format!(
                                "{}\nExpected `{}` arguments, found `{}`",
                                rt.stack_trace(),
                                f.args.len(), args.len()), rt))
                    }
                    try!(lifetimechk::check(f, &args).map_err(|err|
                        module.error(call.args[2].source_range(),
                        &format!("{}\n{}", err, rt.stack_trace()), rt)));
                }
                FnIndex::Intrinsic(_) | FnIndex::None | FnIndex::External(_) =>
                    return Err(module.error(
                        call.args[1].source_range(),
                        &format!(
                            "{}\nCould not find function `{}`",
                            rt.stack_trace(),
                            fn_name), rt))
            }
            let call = ast::Call {
                name: fn_name.clone(),
                f_index: Cell::new(f_index),
                args: args.iter().map(|arg|
                    ast::Expression::Variable(
                        call.source_range, arg.clone())).collect(),
                custom_source: Some(source),
                source_range: call.source_range,
            };

            try!(rt.call(&call, &m)).0
        }
        None => return Err(module.error(call.args[0].source_range(),
            &format!("{}\nExpected `Module`", rt.stack_trace()), rt))
    };

    rt.pop_fn(call.name.clone());
    Ok(v)
}

fn functions(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    // List available functions in scope.
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let mut functions = vec![];
    let name: Arc<String> = Arc::new("name".into());
    let arguments: Arc<String> = Arc::new("arguments".into());
    let returns: Arc<String> = Arc::new("returns".into());
    let takes: Arc<String> = Arc::new("takes".into());
    let lifetime: Arc<String> = Arc::new("lifetime".into());
    let ret_lifetime: Arc<String> = Arc::new("return".into());
    let ty: Arc<String> = Arc::new("type".into());
    let intrinsic: Arc<String> = Arc::new("intrinsic".into());
    let external: Arc<String> = Arc::new("external".into());
    let loaded: Arc<String> = Arc::new("loaded".into());
    let mut intrinsics = Prelude::new();
    standard(&mut intrinsics);
    for (f_name, &f) in &intrinsics.functions {
        let f = &intrinsics.list[f];
        let mut obj = HashMap::new();
        obj.insert(name.clone(), Variable::Text(f_name.clone()));
        obj.insert(returns.clone(), Variable::Text(Arc::new(f.ret.description())));
        obj.insert(ty.clone(), Variable::Text(intrinsic.clone()));
        let mut args = vec![];
        for (i, lt) in f.lts.iter().enumerate() {
            let mut obj_arg = HashMap::new();
            obj_arg.insert(name.clone(),
                Variable::Text(Arc::new(format!("arg{}", i).into())));
            obj_arg.insert(lifetime.clone(), match *lt {
                Lt::Default => Variable::Option(None),
                Lt::Arg(ind) => Variable::Option(Some(
                        Box::new(Variable::Text(
                            Arc::new(format!("arg{}", ind).into())
                        ))
                    )),
                Lt::Return => Variable::Option(Some(
                        Box::new(Variable::Text(ret_lifetime.clone()))
                    )),
            });
            obj_arg.insert(takes.clone(),
                Variable::Text(Arc::new(f.tys[i].description())));
            args.push(Variable::Object(Arc::new(obj_arg)));
        }
        obj.insert(arguments.clone(), Variable::Array(Arc::new(args)));
        functions.push(Variable::Object(Arc::new(obj)));
    }
    for f in &*module.ext_prelude {
        let mut obj = HashMap::new();
        obj.insert(name.clone(), Variable::Text(f.name.clone()));
        obj.insert(returns.clone(), Variable::Text(Arc::new(f.p.ret.description())));
        obj.insert(ty.clone(), Variable::Text(external.clone()));
        let mut args = vec![];
        for (i, lt) in f.p.lts.iter().enumerate() {
            let mut obj_arg = HashMap::new();
            obj_arg.insert(name.clone(),
                Variable::Text(Arc::new(format!("arg{}", i).into())));
            obj_arg.insert(lifetime.clone(), match *lt {
                Lt::Default => Variable::Option(None),
                Lt::Arg(ind) => Variable::Option(Some(
                        Box::new(Variable::Text(
                            Arc::new(format!("arg{}", ind).into())
                        ))
                    )),
                Lt::Return => Variable::Option(Some(
                        Box::new(Variable::Text(ret_lifetime.clone()))
                    )),
            });
            obj_arg.insert(takes.clone(),
                Variable::Text(Arc::new(f.p.tys[i].description())));
            args.push(Variable::Object(Arc::new(obj_arg)));
        }
        obj.insert(arguments.clone(), Variable::Array(Arc::new(args)));
        functions.push(Variable::Object(Arc::new(obj)));
    }
    for f in &module.functions {
        let mut obj = HashMap::new();
        obj.insert(name.clone(), Variable::Text(f.name.clone()));
        obj.insert(returns.clone(), Variable::Text(Arc::new(f.ret.description())));
        obj.insert(ty.clone(), Variable::Text(loaded.clone()));
        let mut args = vec![];
        for arg in &f.args {
            let mut obj_arg = HashMap::new();
            obj_arg.insert(name.clone(),
                Variable::Text(arg.name.clone()));
            obj_arg.insert(lifetime.clone(),
                match arg.lifetime {
                    None => Variable::Option(None),
                    Some(ref lt) => Variable::Option(Some(Box::new(
                            Variable::Text(lt.clone())
                        )))
                }
            );
            obj_arg.insert(takes.clone(),
                Variable::Text(Arc::new(arg.ty.description())));
            args.push(Variable::Object(Arc::new(obj_arg)));
        }
        obj.insert(arguments.clone(), Variable::Array(Arc::new(args)));
        functions.push(Variable::Object(Arc::new(obj)));
    }
    // Sort by function names.
    functions.sort_by(|a, b|
        match (a, b) {
            (&Variable::Object(ref a), &Variable::Object(ref b)) => {
                match (&a[&name], &b[&name]) {
                    (&Variable::Text(ref a), &Variable::Text(ref b)) => {
                        a.cmp(b)
                    }
                    _ => panic!("Expected two strings")
                }
            }
            _ => panic!("Expected two objects")
        }
    );
    let v = Variable::Array(Arc::new(functions));
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn none(
    _rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    Ok(Some(Variable::Option(None)))
}

fn some(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(Variable::Option(Some(Box::new(
        rt.resolve(&v).deep_clone(&rt.stack)
    )))))
}

fn ok(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(Variable::Result(Ok(Box::new(
        rt.resolve(&v).deep_clone(&rt.stack)
    )))))
}

fn err(
    rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(Variable::Result(Err(Box::new(
        Error {
            message: rt.resolve(&v).deep_clone(&rt.stack),
            trace: vec![]
        })))))
}

fn is_err(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Result(Err(_)) => Variable::bool(true),
        &Variable::Result(Ok(_)) => Variable::bool(false),
        x => {
            return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "result"), rt));
        }
    }))
}

fn is_ok(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Result(Err(_)) => Variable::bool(false),
        &Variable::Result(Ok(_)) => Variable::bool(true),
        x => {
            return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "result"), rt));
        }
    }))
}

fn min(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Array(ref arr) => {
            let mut min: f64 = ::std::f64::NAN;
            for v in &**arr {
                if let &Variable::F64(val, _) = rt.resolve(v) {
                    if val < min || min.is_nan() { min = val }
                }
            }
            min
        }
        x => {
            return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "array"), rt));
        }
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::f64(v)))
}

fn max(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Array(ref arr) => {
            let mut max: f64 = ::std::f64::NAN;
            for v in &**arr {
                if let &Variable::F64(val, _) = rt.resolve(v) {
                    if val > max || max.is_nan() { max = val }
                }
            }
            max
        }
        x => {
            return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "array"), rt));
        }
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::f64(v)))
}

fn unwrap(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use write::{write_variable, EscapeString};

    // Return value does not depend on lifetime of argument since
    // `ok(x)` and `some(x)` perform a deep clone.
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Option(Some(ref v)) => (**v).clone(),
        &Variable::Option(None) => {
            return Err(module.error(call.args[0].source_range(),
                &format!("{}\nExpected `some(_)`",
                    rt.stack_trace()), rt));
        }
        &Variable::Result(Ok(ref ok)) => (**ok).clone(),
        &Variable::Result(Err(ref err)) => {
            use std::str::from_utf8;

            // Print out error message.
            let mut w: Vec<u8> = vec![];
            w.extend_from_slice(rt.stack_trace().as_bytes());
            w.extend_from_slice("\n".as_bytes());
            write_variable(&mut w, rt, &err.message,
                           EscapeString::None, 0).unwrap();
            for t in &err.trace {
                w.extend_from_slice("\n".as_bytes());
                w.extend_from_slice(t.as_bytes());
            }
            return Err(module.error(call.args[0].source_range(),
                                    from_utf8(&w).unwrap(), rt));
        }
        x => {
            return Err(module.error(call.args[0].source_range(),
                                    &rt.expected(x, "some(_) or ok(_)"), rt));
        }
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn unwrap_or(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    // Return value does not depend on lifetime of argument since
    // `ok(x)` and `some(x)` perform a deep clone.
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let def = rt.stack.pop().expect(TINVOTS);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Option(Some(ref v)) => (**v).clone(),
        &Variable::Result(Ok(ref ok)) => (**ok).clone(),
        &Variable::Option(None) |
        &Variable::Result(Err(_)) => rt.resolve(&def).clone(),
        x => {
            return Err(module.error(call.args[0].source_range(),
                                    &rt.expected(x, "some(_) or ok(_)"), rt));
        }
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn unwrap_err(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::Result(Err(ref err)) => err.message.clone(),
        x => {
            return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "err(_)"), rt));
        }
    }))
}

fn dir__angle(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize,
) -> Result<Option<Variable>, String> {
    let v = rt.stack.pop().expect(TINVOTS);
    Ok(Some(match rt.resolve(&v) {
        &Variable::F64(val, _) => Variable::Vec4([val.cos() as f32, val.sin() as f32, 0.0, 0.0]),
        x => {
            return Err(module.error(call.args[0].source_range(),
                &rt.expected(x, "err(_)"), rt));
        }
    }))
}

fn load__meta_file(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let file = rt.stack.pop().expect(TINVOTS);
    let meta = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref file) => file.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let meta = match rt.resolve(&meta) {
        &Variable::Text(ref meta) => meta.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let res = meta::load_meta_file(&**meta, &**file);
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Array(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })))
}

fn load__meta_url(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let url = rt.stack.pop().expect(TINVOTS);
    let meta = rt.stack.pop().expect(TINVOTS);
    let url = match rt.resolve(&url) {
        &Variable::Text(ref url) => url.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let meta = match rt.resolve(&meta) {
        &Variable::Text(ref meta) => meta.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let res = meta::load_meta_url(&**meta, &**url);
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Array(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })))
}

fn download__url_file(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let file = rt.stack.pop().expect(TINVOTS);
    let url = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref file) => file.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let url = match rt.resolve(&url) {
        &Variable::Text(ref url) => url.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "str"), rt))
    };

    let res = meta::download_url_to_file(&**url, &**file);
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Text(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })))
}

fn save__string_file(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use std::fs::File;
    use std::io::Write;
    use std::error::Error as StdError;

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let file = rt.stack.pop().expect(TINVOTS);
    let text = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref file) => file.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let text = match rt.resolve(&text) {
        &Variable::Text(ref text) => text.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "str"), rt))
    };

    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(match File::create(&**file) {
        Ok(mut f) => {
            match f.write_all(text.as_bytes()) {
                Ok(_) => Ok(Box::new(Variable::Text(file))),
                Err(err) => Err(Box::new(Error {
                    message: Variable::Text(Arc::new(err.description().into())),
                    trace: vec![]
                }))
            }
        }
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err.description().into())),
            trace: vec![]
        }))
    })))
}

fn load_string__file(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use std::fs::File;
    use std::io::Read;
    use std::error::Error as StdError;

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let file = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref file) => file.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "str"), rt))
    };

    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(match File::open(&**file) {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => {
                    Ok(Box::new(Variable::Text(Arc::new(s))))
                }
                Err(err) => {
                    Err(Box::new(Error {
                        message: Variable::Text(Arc::new(err.description().into())),
                        trace: vec![]
                    }))
                }
            }
        }
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err.description().into())),
            trace: vec![]
        }))
    })))
}

fn join__thread(
    rt: &mut Runtime,
    call: &ast::Call,
    _module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use Thread;

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let thread = rt.stack.pop().expect(TINVOTS);
    let handle_res = Thread::invalidate_handle(rt, thread);
    let v = Variable::Result({
        match handle_res {
            Ok(handle) => {
                match handle.join() {
                    Ok(res) => match res {
                        Ok(res) => Ok(Box::new(res)),
                        Err(err) => Err(Box::new(Error {
                            message: Variable::Text(Arc::new(err)),
                            trace: vec![]
                        }))
                    },
                    Err(_err) => Err(Box::new(Error {
                        message: Variable::Text(Arc::new(
                            "Thread did not exit successfully".into())),
                        trace: vec![]
                    }))
                }
            }
            Err(err) => {
                Err(Box::new(Error {
                    message: Variable::Text(Arc::new(err)),
                    trace: vec![]
                }))
            }
        }
    });
    rt.pop_fn(call.name.clone());
    Ok(Some(v))
}

fn load_data__file(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let file = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "string"), rt))
    };
    let res = match data::load_file(&file) {
        Ok(data) => Ok(Box::new(data)),
        Err(err) => Err(Box::new(super::Error {
            message: Variable::Text(Arc::new(format!(
                        "Error loading data from file `{}`:\n{}",
                        file, err))),
            trace: vec![]
        }))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(res)))
}

fn save__data_file(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use std::error::Error;
    use std::fs::File;
    use std::io::BufWriter;
    use write::{write_variable, EscapeString};

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let file = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "string"), rt))
    };
    let data = rt.stack.pop().expect(TINVOTS);

    let mut f = match File::create(&**file) {
        Ok(f) => BufWriter::new(f),
        Err(err) => {
            return Err(module.error(call.args[0].source_range(),
                       &format!("{}\nError when creating file `{}`:\n{}",
                        rt.stack_trace(), file, err.description()), rt))
        }
    };
    let res = match write_variable(&mut f, rt, &data, EscapeString::Json, 0) {
        Ok(()) => Ok(Box::new(Variable::Text(file.clone()))),
        Err(err) => {
            Err(Box::new(super::Error {
                message: Variable::Text(Arc::new(format!(
                            "Error when writing to file `{}`:\n{}",
                            file, err.description()))),
                trace: vec![]
            }))
        }
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Result(res)))
}

fn json_from_meta_data(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    use std::error::Error;

    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let meta_data = rt.stack.pop().expect(TINVOTS);
    let json = match rt.resolve(&meta_data) {
        &Variable::Array(ref arr) => {
            try!(meta::json_from_meta_data(arr).map_err(|err| {
                format!("{}\nError when generating JSON:\n{}",
                        rt.stack_trace(),
                        err.description())
            }))
        }
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "array"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Text(Arc::new(json))))
}

fn has(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let key = rt.stack.pop().expect(TINVOTS);
    let key = match rt.resolve(&key) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[1].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let obj = rt.stack.pop().expect(TINVOTS);
    let res = match rt.resolve(&obj) {
        &Variable::Object(ref obj) => obj.contains_key(&key),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "array"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::bool(res)))
}

fn chars(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let t = rt.stack.pop().expect(TINVOTS);
    let t = match rt.resolve(&t) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "str"), rt))
    };
    let res = t.chars()
        .map(|ch| {
            let mut s = String::new();
            s.push(ch);
            Variable::Text(Arc::new(s))
        })
        .collect::<Vec<_>>();
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::Array(Arc::new(res))))
}

fn now(
    _rt: &mut Runtime,
    _call: &ast::Call,
    _module: &Module,
    _st: usize,
    _lc: usize,
    _cu: usize
) -> Result<Option<Variable>, String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let val = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => Variable::f64(val.as_secs() as f64 +
                                 val.subsec_nanos() as f64 / 1.0e9),
        Err(err) => Variable::f64(-{
            let val = err.duration();
            val.as_secs() as f64 +
            val.subsec_nanos() as f64 / 1.0e9
        })
    };
    Ok(Some(val))
}

fn is_nan(
    rt: &mut Runtime,
    call: &ast::Call,
    module: &Module,
    st: usize,
    lc: usize,
    cu: usize,
) -> Result<Option<Variable>, String> {
    rt.push_fn(call.name.clone(), 0, None, st + 1, lc, cu);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::F64(ref v, _) => v.clone(),
        x => return Err(module.error(call.args[0].source_range(),
                        &rt.expected(x, "number"), rt))
    };
    rt.pop_fn(call.name.clone());
    Ok(Some(Variable::bool(v.is_nan())))
}
