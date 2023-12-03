// --- Day 7: Some Assembly Required ---
// This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates!
// Unfortunately, little Bobby is a little under the recommended age range, and he needs help
// assembling the circuit.
//
// Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from
// 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value.
// Each wire can only get a signal from one source, but can provide its signal to multiple
// destinations. A gate provides no signal until all of its inputs have a signal.
//
// The included instructions booklet describes how to connect the parts together: x AND y -> z means
// to connect wires x and y to an AND gate, and then connect its output to wire z.
//
// For example:
//
// - 123 -> x means that the signal 123 is provided to wire x.
// - x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
// - p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
// - NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.
//
// Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for some reason, you'd
// like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript,
// or Python) provide operators for these gates.
//
// For example, here is a simple circuit:
//
//   123 -> x
//   456 -> y
//   x AND y -> d
//   x OR y -> e
//   x LSHIFT 2 -> f
//   y RSHIFT 2 -> g
//   NOT x -> h
//   NOT y -> i
//
// After it is run, these are the signals on the wires:
//
//   d: 72
//   e: 507
//   f: 492
//   g: 114
//   h: 65412
//   i: 65079
//   x: 123
//   y: 456
//
// In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is
// ultimately provided to wire a?
//
// Your puzzle answer was 16076.
//
// --- Part Two ---
// Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires
// (including wire a). What new signal is ultimately provided to wire a?
//
// Your puzzle answer was 2797.

use indoc::indoc;
use regex::Regex;
use std::collections::HashMap;
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 7: Some Assembly Required ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("In little Bobby's kit's instructions booklet, what signal is ultimately provided to wire a?");
    println!(" {}", answer_a);
    println!(" in {}ms", duration.as_millis());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}

fn part_a() -> u16 {
    let dr = DayRegex::init();

    let mut input_map: HashMap<String, Gate> = HashMap::new();
    for line in INPUT_A.lines() {
        if let Some((k, g)) = Gate::from(line, &dr) {
            input_map.insert(k, g);
        }
    }

    let mut eval_map: HashMap<String, u16> = HashMap::new();
    let wire = "a";
    solve(&input_map, &mut eval_map, wire);
    let p_answer = eval_map.get(wire).unwrap();
    *p_answer
}

fn part_b() -> u16 {
    let dr = DayRegex::init();

    let mut input_map: HashMap<String, Gate> = HashMap::new();
    for line in INPUT_A.lines() {
        if let Some((k, g)) = Gate::from(line, &dr) {
            input_map.insert(k, g);
        }
    }

    let mut eval_map: HashMap<String, u16> = HashMap::new();
    solve(&input_map, &mut eval_map, "a");
    let a_answer = *(eval_map.get("a").unwrap());

    // Now create a new B value based on what A is. Override the input map with the new value
    // for B, clear the eval map and solve again for a
    let new_b = Gate::Value(a_answer);
    input_map.insert(String::from("b"), new_b);
    eval_map.clear();

    solve(&input_map, &mut eval_map, "a");
    *(eval_map.get("a").unwrap())
}

fn solve(input_map: &HashMap<String, Gate>, eval_map: &mut HashMap<String, u16>, wire: &str) {
    if !eval_map.contains_key(wire) {
        let gate = input_map
            .get(wire)
            .unwrap_or_else(|| panic!("Missing gate for wire: {wire}"));
        match gate {
            Gate::Value(v) => {
                eval_map.insert(wire.to_string(), *v);
            }
            Gate::Wire(wire_a) => {
                solve(input_map, eval_map, wire_a);
                let v = *(eval_map.get(wire_a).unwrap());
                eval_map.insert(wire.to_string(), v);
            }
            Gate::And(wire_a, wire_b) => {
                let v_a = lit_or_solve(input_map, eval_map, wire_a);
                let v_b = lit_or_solve(input_map, eval_map, wire_b);
                let v = v_a & v_b;
                eval_map.insert(wire.to_string(), v);
            }
            Gate::Or(wire_a, wire_b) => {
                let v_a = lit_or_solve(input_map, eval_map, wire_a);
                let v_b = lit_or_solve(input_map, eval_map, wire_b);
                let v = v_a | v_b;
                eval_map.insert(wire.to_string(), v);
            }
            Gate::LShift(wire_a, shift) => {
                let v_a = lit_or_solve(input_map, eval_map, wire_a);
                let v = v_a << shift;
                eval_map.insert(wire.to_string(), v);
            }
            Gate::RShift(wire_a, shift) => {
                let v_a = lit_or_solve(input_map, eval_map, wire_a);
                let v = v_a >> shift;
                eval_map.insert(wire.to_string(), v);
            }
            Gate::Not(wire_a) => {
                let v_a = lit_or_solve(input_map, eval_map, wire_a);
                let v = !v_a;
                eval_map.insert(wire.to_string(), v);
            }
        };
    }
}

fn lit_or_solve(
    input_map: &HashMap<String, Gate>,
    eval_map: &mut HashMap<String, u16>,
    wire: &str,
) -> u16 {
    // Literal value or solve
    match wire.parse::<u16>() {
        Ok(literal) => literal,
        Err(_) => {
            solve(input_map, eval_map, wire);
            *(eval_map.get(wire).unwrap())
        }
    }
}

#[derive(Debug)]
enum Gate {
    Value(u16),
    Wire(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
    Not(String),
}

impl Gate {
    fn from(s: &str, dr: &DayRegex) -> Option<(String, Gate)> {
        let ret;
        if let Some(g) = dr.parse_value(s) {
            ret = Some(g);
        } else if let Some(g) = dr.parse_wire(s) {
            ret = Some(g);
        } else if let Some(g) = dr.parse_and(s) {
            ret = Some(g);
        } else if let Some(g) = dr.parse_or(s) {
            ret = Some(g);
        } else if let Some(g) = dr.parse_lshift(s) {
            ret = Some(g);
        } else if let Some(g) = dr.parse_rshift(s) {
            ret = Some(g);
        } else if let Some(g) = dr.parse_not(s) {
            ret = Some(g);
        } else {
            ret = None;
        };
        ret
    }
}

struct DayRegex {
    value: Regex,
    wire: Regex,
    and: Regex,
    or: Regex,
    lshift: Regex,
    rshift: Regex,
    not: Regex,
}

impl DayRegex {
    fn init() -> DayRegex {
        DayRegex {
            value: Regex::new(r"^(?<value>[0-9]+) -> (?<key>[a-z]+)$").unwrap(), // 123 -> x
            wire: Regex::new(r"^(?<wire>[a-z]+) -> (?<key>[a-z]+)$").unwrap(),   // y -> x
            and: Regex::new(r"^(?<wire_a>[0-9a-z]+) AND (?<wire_b>[0-9a-z]+) -> (?<key>[a-z]+)$")
                .unwrap(), // lf AND lq -> ls
            or: Regex::new(r"^(?<wire_a>[0-9a-z]+) OR (?<wire_b>[0-9a-z]+) -> (?<key>[a-z]+)$")
                .unwrap(), // kl OR kr -> ks
            lshift: Regex::new(r"^(?<wire>[a-z]+) LSHIFT (?<value>[0-9]+) -> (?<key>[a-z]+)$")
                .unwrap(), // fj LSHIFT 15 -> fn
            rshift: Regex::new(r"^(?<wire>[a-z]+) RSHIFT (?<value>[0-9]+) -> (?<key>[a-z]+)$")
                .unwrap(), // fj RSHIFT 15 -> fn
            not: Regex::new(r"^NOT (?<wire>[a-z]+) -> (?<key>[a-z]+)$").unwrap(),
        }
    }

    fn parse_value(&self, s: &str) -> Option<(String, Gate)> {
        self.value.captures(s).map(|c| {
            let value = c.name("value").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let v: u16 = value.parse().unwrap();
            (k, Gate::Value(v))
        })
    }

    fn parse_wire(&self, s: &str) -> Option<(String, Gate)> {
        self.wire.captures(s).map(|c| {
            let value = c.name("wire").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let w = String::from(value);
            (k, Gate::Wire(w))
        })
    }

    fn parse_and(&self, s: &str) -> Option<(String, Gate)> {
        self.and.captures(s).map(|c| {
            let wire_a = c.name("wire_a").unwrap().as_str();
            let wire_b = c.name("wire_b").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let wa = String::from(wire_a);
            let wb = String::from(wire_b);
            (k, Gate::And(wa, wb))
        })
    }

    fn parse_or(&self, s: &str) -> Option<(String, Gate)> {
        self.or.captures(s).map(|c| {
            let wire_a = c.name("wire_a").unwrap().as_str();
            let wire_b = c.name("wire_b").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let wa = String::from(wire_a);
            let wb = String::from(wire_b);
            (k, Gate::Or(wa, wb))
        })
    }

    fn parse_lshift(&self, s: &str) -> Option<(String, Gate)> {
        self.lshift.captures(s).map(|c| {
            let wire = c.name("wire").unwrap().as_str();
            let value = c.name("value").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let w = String::from(wire);
            let v: u16 = value.parse().unwrap();
            (k, Gate::LShift(w, v))
        })
    }

    fn parse_rshift(&self, s: &str) -> Option<(String, Gate)> {
        self.rshift.captures(s).map(|c| {
            let wire = c.name("wire").unwrap().as_str();
            let value = c.name("value").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let w = String::from(wire);
            let v: u16 = value.parse().unwrap();
            (k, Gate::RShift(w, v))
        })
    }

    fn parse_not(&self, s: &str) -> Option<(String, Gate)> {
        self.not.captures(s).map(|c| {
            let value = c.name("wire").unwrap().as_str();
            let key = c.name("key").unwrap().as_str();

            let k = String::from(key);
            let w = String::from(value);
            (k, Gate::Not(w))
        })
    }
}

const INPUT_A: &str = indoc! {r#"
lf AND lq -> ls
iu RSHIFT 1 -> jn
bo OR bu -> bv
gj RSHIFT 1 -> hc
et RSHIFT 2 -> eu
bv AND bx -> by
is OR it -> iu
b OR n -> o
gf OR ge -> gg
NOT kt -> ku
ea AND eb -> ed
kl OR kr -> ks
hi AND hk -> hl
au AND av -> ax
lf RSHIFT 2 -> lg
dd RSHIFT 3 -> df
eu AND fa -> fc
df AND dg -> di
ip LSHIFT 15 -> it
NOT el -> em
et OR fe -> ff
fj LSHIFT 15 -> fn
t OR s -> u
ly OR lz -> ma
ko AND kq -> kr
NOT fx -> fy
et RSHIFT 1 -> fm
eu OR fa -> fb
dd RSHIFT 2 -> de
NOT go -> gp
kb AND kd -> ke
hg OR hh -> hi
jm LSHIFT 1 -> kg
NOT cn -> co
jp RSHIFT 2 -> jq
jp RSHIFT 5 -> js
1 AND io -> ip
eo LSHIFT 15 -> es
1 AND jj -> jk
g AND i -> j
ci RSHIFT 3 -> ck
gn AND gp -> gq
fs AND fu -> fv
lj AND ll -> lm
jk LSHIFT 15 -> jo
iu RSHIFT 3 -> iw
NOT ii -> ij
1 AND cc -> cd
bn RSHIFT 3 -> bp
NOT gw -> gx
NOT ft -> fu
jn OR jo -> jp
iv OR jb -> jc
hv OR hu -> hw
19138 -> b
gj RSHIFT 5 -> gm
hq AND hs -> ht
dy RSHIFT 1 -> er
ao OR an -> ap
ld OR le -> lf
bk LSHIFT 1 -> ce
bz AND cb -> cc
bi LSHIFT 15 -> bm
il AND in -> io
af AND ah -> ai
as RSHIFT 1 -> bl
lf RSHIFT 3 -> lh
er OR es -> et
NOT ax -> ay
ci RSHIFT 1 -> db
et AND fe -> fg
lg OR lm -> ln
k AND m -> n
hz RSHIFT 2 -> ia
kh LSHIFT 1 -> lb
NOT ey -> ez
NOT di -> dj
dz OR ef -> eg
lx -> a
NOT iz -> ja
gz LSHIFT 15 -> hd
ce OR cd -> cf
fq AND fr -> ft
at AND az -> bb
ha OR gz -> hb
fp AND fv -> fx
NOT gb -> gc
ia AND ig -> ii
gl OR gm -> gn
0 -> c
NOT ca -> cb
bn RSHIFT 1 -> cg
c LSHIFT 1 -> t
iw OR ix -> iy
kg OR kf -> kh
dy OR ej -> ek
km AND kn -> kp
NOT fc -> fd
hz RSHIFT 3 -> ib
NOT dq -> dr
NOT fg -> fh
dy RSHIFT 2 -> dz
kk RSHIFT 2 -> kl
1 AND fi -> fj
NOT hr -> hs
jp RSHIFT 1 -> ki
bl OR bm -> bn
1 AND gy -> gz
gr AND gt -> gu
db OR dc -> dd
de OR dk -> dl
as RSHIFT 5 -> av
lf RSHIFT 5 -> li
hm AND ho -> hp
cg OR ch -> ci
gj AND gu -> gw
ge LSHIFT 15 -> gi
e OR f -> g
fp OR fv -> fw
fb AND fd -> fe
cd LSHIFT 15 -> ch
b RSHIFT 1 -> v
at OR az -> ba
bn RSHIFT 2 -> bo
lh AND li -> lk
dl AND dn -> do
eg AND ei -> ej
ex AND ez -> fa
NOT kp -> kq
NOT lk -> ll
x AND ai -> ak
jp OR ka -> kb
NOT jd -> je
iy AND ja -> jb
jp RSHIFT 3 -> jr
fo OR fz -> ga
df OR dg -> dh
gj RSHIFT 2 -> gk
gj OR gu -> gv
NOT jh -> ji
ap LSHIFT 1 -> bj
NOT ls -> lt
ir LSHIFT 1 -> jl
bn AND by -> ca
lv LSHIFT 15 -> lz
ba AND bc -> bd
cy LSHIFT 15 -> dc
ln AND lp -> lq
x RSHIFT 1 -> aq
gk OR gq -> gr
NOT kx -> ky
jg AND ji -> jj
bn OR by -> bz
fl LSHIFT 1 -> gf
bp OR bq -> br
he OR hp -> hq
et RSHIFT 5 -> ew
iu RSHIFT 2 -> iv
gl AND gm -> go
x OR ai -> aj
hc OR hd -> he
lg AND lm -> lo
lh OR li -> lj
da LSHIFT 1 -> du
fo RSHIFT 2 -> fp
gk AND gq -> gs
bj OR bi -> bk
lf OR lq -> lr
cj AND cp -> cr
hu LSHIFT 15 -> hy
1 AND bh -> bi
fo RSHIFT 3 -> fq
NOT lo -> lp
hw LSHIFT 1 -> iq
dd RSHIFT 1 -> dw
dt LSHIFT 15 -> dx
dy AND ej -> el
an LSHIFT 15 -> ar
aq OR ar -> as
1 AND r -> s
fw AND fy -> fz
NOT im -> in
et RSHIFT 3 -> ev
1 AND ds -> dt
ec AND ee -> ef
NOT ak -> al
jl OR jk -> jm
1 AND en -> eo
lb OR la -> lc
iu AND jf -> jh
iu RSHIFT 5 -> ix
bo AND bu -> bw
cz OR cy -> da
iv AND jb -> jd
iw AND ix -> iz
lf RSHIFT 1 -> ly
iu OR jf -> jg
NOT dm -> dn
lw OR lv -> lx
gg LSHIFT 1 -> ha
lr AND lt -> lu
fm OR fn -> fo
he RSHIFT 3 -> hg
aj AND al -> am
1 AND kz -> la
dy RSHIFT 5 -> eb
jc AND je -> jf
cm AND co -> cp
gv AND gx -> gy
ev OR ew -> ex
jp AND ka -> kc
fk OR fj -> fl
dy RSHIFT 3 -> ea
NOT bs -> bt
NOT ag -> ah
dz AND ef -> eh
cf LSHIFT 1 -> cz
NOT cv -> cw
1 AND cx -> cy
de AND dk -> dm
ck AND cl -> cn
x RSHIFT 5 -> aa
dv LSHIFT 1 -> ep
he RSHIFT 2 -> hf
NOT bw -> bx
ck OR cl -> cm
bp AND bq -> bs
as OR bd -> be
he AND hp -> hr
ev AND ew -> ey
1 AND lu -> lv
kk RSHIFT 3 -> km
b AND n -> p
NOT kc -> kd
lc LSHIFT 1 -> lw
km OR kn -> ko
id AND if -> ig
ih AND ij -> ik
jr AND js -> ju
ci RSHIFT 5 -> cl
hz RSHIFT 1 -> is
1 AND ke -> kf
NOT gs -> gt
aw AND ay -> az
x RSHIFT 2 -> y
ab AND ad -> ae
ff AND fh -> fi
ci AND ct -> cv
eq LSHIFT 1 -> fk
gj RSHIFT 3 -> gl
u LSHIFT 1 -> ao
NOT bb -> bc
NOT hj -> hk
kw AND ky -> kz
as AND bd -> bf
dw OR dx -> dy
br AND bt -> bu
kk AND kv -> kx
ep OR eo -> eq
he RSHIFT 1 -> hx
ki OR kj -> kk
NOT ju -> jv
ek AND em -> en
kk RSHIFT 5 -> kn
NOT eh -> ei
hx OR hy -> hz
ea OR eb -> ec
s LSHIFT 15 -> w
fo RSHIFT 1 -> gh
kk OR kv -> kw
bn RSHIFT 5 -> bq
NOT ed -> ee
1 AND ht -> hu
cu AND cw -> cx
b RSHIFT 5 -> f
kl AND kr -> kt
iq OR ip -> ir
ci RSHIFT 2 -> cj
cj OR cp -> cq
o AND q -> r
dd RSHIFT 5 -> dg
b RSHIFT 2 -> d
ks AND ku -> kv
b RSHIFT 3 -> e
d OR j -> k
NOT p -> q
NOT cr -> cs
du OR dt -> dv
kf LSHIFT 15 -> kj
NOT ac -> ad
fo RSHIFT 5 -> fr
hz OR ik -> il
jx AND jz -> ka
gh OR gi -> gj
kk RSHIFT 1 -> ld
hz RSHIFT 5 -> ic
as RSHIFT 2 -> at
NOT jy -> jz
1 AND am -> an
ci OR ct -> cu
hg AND hh -> hj
jq OR jw -> jx
v OR w -> x
la LSHIFT 15 -> le
dh AND dj -> dk
dp AND dr -> ds
jq AND jw -> jy
au OR av -> aw
NOT bf -> bg
z OR aa -> ab
ga AND gc -> gd
hz AND ik -> im
jt AND jv -> jw
z AND aa -> ac
jr OR js -> jt
hb LSHIFT 1 -> hv
hf OR hl -> hm
ib OR ic -> id
fq OR fr -> fs
cq AND cs -> ct
ia OR ig -> ih
dd OR do -> dp
d AND j -> l
ib AND ic -> ie
as RSHIFT 3 -> au
be AND bg -> bh
dd AND do -> dq
NOT l -> m
1 AND gd -> ge
y AND ae -> ag
fo AND fz -> gb
NOT ie -> if
e AND f -> h
x RSHIFT 3 -> z
y OR ae -> af
hf AND hl -> hn
NOT h -> i
NOT hn -> ho
he RSHIFT 5 -> hh"#};

#[test]
fn test_a() {
    let result = part_a();
    assert_eq!(result, 16076);
}

#[test]
fn test_b() {
    let result = part_b();
    assert_eq!(result, 2797);
}
