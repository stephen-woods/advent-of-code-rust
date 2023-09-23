// --- Day 8: Matchsticks ---
// Space on the sleigh is limited this year, and so Santa will be bringing his list as a digital
// copy. He needs to know how much space it will take up when stored.
//
// It is common in many programming languages to provide a way to escape special characters in
// strings. For example, C, JavaScript, Perl, Python, and even PHP handle special characters in very
// similar ways.
//
// However, it is important to realize the difference between the number of characters in the code
// representation of the string literal and the number of characters in the in-memory string itself.
//
// For example:
//
// - "" is 2 characters of code (the two double quotes), but the string contains zero characters.
// - "abc" is 5 characters of code, but 3 characters in the string data.
// - "aaa\"aaa" is 10 characters of code, but the string itself contains six "a" characters and a
//   single, escaped quote character, for a total of 7 characters in the string data.
// - "\x27" is 6 characters of code, but the string itself contains just one - an apostrophe ('),
//   escaped using hexadecimal notation.
//
// Santa's list is a file that contains many double-quoted string literals, one on each line. The
// only escape sequences used are \\ (which represents a single backslash), \" (which represents a
// lone double-quote character), and \x plus two hexadecimal characters (which represents a single
// character with that ASCII code).
//
// Disregarding the whitespace in the file, what is the number of characters of code for string
// literals minus the number of characters in memory for the values of the strings in total for the
// entire file?
//
// For example, given the four strings above, the total number of characters of string code
// (2 + 5 + 10 + 6 = 23) minus the total number of characters in memory for string values
// (0 + 3 + 7 + 1 = 11) is 23 - 11 = 12.
//
// Your puzzle answer was 1342.
//
// --- Part Two ---
// Now, let's go the other way. In addition to finding the number of characters of code, you should
// now encode each code representation as a new string and find the number of characters of the new
// encoded representation, including the surrounding double quotes.
//
// For example:
//
// -  "" encodes to "\"\"", an increase from 2 characters to 6.
// -  "abc" encodes to "\"abc\"", an increase from 5 characters to 9.
// -  "aaa\"aaa" encodes to "\"aaa\\\"aaa\"", an increase from 10 characters to 16.
// -  "\x27" encodes to "\"\\x27\"", an increase from 6 characters to 11.
//
// Your task is to find the total number of characters to represent the newly encoded strings minus
// the number of characters of code in each original string literal. For example, for the strings
// above, the total encoded length (6 + 9 + 16 + 11 = 42) minus the characters in the original code
// representation (23, just like in the first part of this puzzle) is 42 - 23 = 19.
//
// Your puzzle answer was 2074.

use indoc::indoc;
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 8: Matchsticks ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Disregarding the whitespace in the file, what is the number of characters of code for string literals minus the number of characters in memory for the values of the strings in total for the entire file?");
    println!(" {}", answer_a);
    println!(" in {}ns", duration.as_nanos());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Find the total number of characters to represent the newly encoded strings minus the number of characters of code in each original string literal");
    println!(" {}", answer_b);
    println!(" in {}ns", duration.as_nanos());
}


fn part_a() -> usize {
    let mut a_answer = 0;
    for line in INPUT_A.lines() {
        a_answer += code_length(line);
        a_answer -= in_memory_length(line);
    }
    a_answer
}


fn part_b() -> usize {
    let mut b_answer = 0;
    for line in INPUT_A.lines() {
        b_answer += encoded_length(line);
        b_answer -= code_length(line);
    }
    b_answer
}


fn code_length(s: &str) -> usize {
    s.len()
}


fn in_memory_length(s: &str) -> usize {
    let s_index = 1;
    let e_index = s.len() - 1;
    let sample = s[s_index..e_index].as_bytes();

    let mut length = 0;
    let mut i = 0;
    let sample_length = sample.len();
    while i != sample_length {
        if sample[i] == b'\\' {
            if sample[i + 1] == b'x' {
                i += 4;
            } else {
                i += 2;
            }
        } else {
            i += 1;
        }
        length += 1;
    }
    length
}


fn encoded_length(s: &str) -> usize {
    let mut length = 0;
    let mut i = 0;
    let sample = s.as_bytes();
    let sample_length = sample.len();
    while i != sample_length {
        let c = sample[i];
        if c == b'"' || c == b'\\'  {
            length += 2;
        } else {
            length += 1;
        }
        i += 1;
    }
    length + 2
}

const _INPUT_SAMPLE: &str = indoc! {r#"
""
"abc"
"aaa\"aaa"
"\x27""#};

const INPUT_A: &str = indoc! {r#"
"azlgxdbljwygyttzkfwuxv"
"v\xfb\"lgs\"kvjfywmut\x9cr"
"merxdhj"
"dwz"
"d\\gkbqo\\fwukyxab\"u"
"k\xd4cfixejvkicryipucwurq\x7eq"
"nvtidemacj\"hppfopvpr"
"kbngyfvvsdismznhar\\p\"\"gpryt\"jaeh"
"khre\"o\x0elqfrbktzn"
"nugkdmqwdq\x50amallrskmrxoyo"
"jcrkptrsasjp\\\"cwigzynjgspxxv\\vyb"
"ramf\"skhcmenhbpujbqwkltmplxygfcy"
"aqjqgbfqaxga\\fkdcahlfi\"pvods"
"pcrtfb"
"\x83qg\"nwgugfmfpzlrvty\"ryoxm"
"fvhvvokdnl\\eap"
"kugdkrat"
"seuxwc"
"vhioftcosshaqtnz"
"gzkxqrdq\\uko\"mrtst"
"znjcomvy\x16hhsenmroswr"
"clowmtra"
"\xc4"
"jpavsevmziklydtqqm"
"egxjqytcttr\\ecfedmmovkyn\"m"
"mjulrvqgmsvmwf"
"o\\prxtlfbatxerhev\xf9hcl\x44rzmvklviv"
"lregjexqaqgwloydxdsc\\o\"dnjfmjcu"
"lnxluajtk\x8desue\\k\x7abhwokfhh"
"wrssfvzzn\"llrysjgiu\"npjtdli"
"\x67lwkks"
"bifw\"ybvmwiyi\"vhol\"vol\xd4"
"aywdqhvtvcpvbewtwuyxrix"
"gc\xd3\"caukdgfdywj"
"uczy\\fk"
"bnlxkjvl\x7docehufkj\\\"qoyhag"
"bidsptalmoicyorbv\\"
"jorscv\"mufcvvfmcv\"ga"
"sofpwfal\\a"
"kcuqtbboaly\"uj\"k"
"n\\c"
"x\"\xcaj\\xwwvpdldz"
"eyukphh"
"wcyjq"
"vjx\"\"hjroj\"l\x4cjwbr"
"xcodsxzfqw\\rowqtuwvjnxupjnrh"
"yc"
"fpvzldgbdtca\"hqwa"
"ymjq\x8ahohvafubra\"hgqoknkuyph"
"kx\\mkaaklvcup"
"belddrzegcsxsyfhzyz"
"fuyswi"
"\\hubzebo\"ha\\qyr\"dv\\"
"mxvlz\"fwuvx\"cyk\""
"ftbh\"ro\\tmcpnpvh\"xx"
"ygi"
"rw\"\"wwn\\fgbjumq\"vgvoh\xd0\"mm"
"\"pat\"\x63kpfc\"\x2ckhfvxk\"uwqzlx"
"o"
"d\"hqtsfp\xceaswe\"\xc0lw"
"zajpvfawqntvoveal\"\"trcdarjua"
"xzapq"
"rkmhm"
"byuq"
"rwwmt\xe8jg\xc2\"omt"
"nfljgdmgefvlh\"x"
"rpjxcexisualz"
"doxcycmgaiptvd"
"rq\\\"mohnjdf\\xv\\hrnosdtmvxot"
"oqvbcenib\"uhy\\npjxg"
"pkvgnm\\ruayuvpbpd"
"kknmzpxqfbcdgng"
"piduhbmaympxdexz"
"vapczawekhoa\\or"
"tlwn\"avc\"bycg\"\"xuxea"
"\xcdvryveteqzxrgopmdmihkcgsuozips"
"kpzziqt"
"sdy\\s\"cjq"
"yujs"
"qte\"q"
"qyvpnkhjcqjv\"cclvv\"pclgtg\xeak\"tno"
"xwx"
"vibuvv"
"qq\""
"wwjduomtbkbdtorhpyalxswisq\"r"
"afuw\\mfjzctcivwesutxbk\"lk"
"e\xcef\\hkiu"
"ftdrgzvygcw\"jwsrcmgxj"
"zrddqfkx\x21dr\"ju\"elybk\"powj\"\"kpryz"
"dttdkfvbodkma\""
"lzygktugpqw"
"qu\x83tes\\u\"tnid\"ryuz"
"\\o\"pe\\vqwlsizjklwrjofg\xe2oau\\rd"
"mikevjzhnwgx\"fozrj\"h\""
"ligxmxznzvtachvvbahnff"
"d\\kq"
"tnbkxpzmcakqhaa"
"g\\yeakebeyv"
"cqkcnd\"sxjxfnawy\x31zax\x6ceha"
"m\x0dtqotffzdnetujtsgjqgwddc"
"masnugb\"etgmxul\x3bqd\\tmtddnvcy"
"floediikodfgre\x23wyoxlswxflwecdjpt"
"zu"
"r"
"\"ashzdbd\"pdvba\xeeumkr\\amnj"
"ckslmuwbtfouwpfwtuiqmeozgspwnhx"
"t\\qjsjek\xf9gjcxsyco\"r"
"hoed\x1b\\tcmaqch\"epdy"
"mgjiojwzc\\ypqcn\xb1njmp\"aeeblxt"
"\xdf\"h\x5enfracj"
"\x6fpbpocrb"
"jbmhrswyyq\\"
"wtyqtenfwatji\"ls\\"
"voy"
"awj"
"rtbj\"j"
"hynl"
"orqqeuaat\\xu\\havsgr\xc5qdk"
"g\"npyzjfq\"rjefwsk"
"rk\\kkcirjbixr\\zelndx\"bsnqvqj\""
"tecoz"
"dn\"uswngbdk\""
"qb\\"
"wpyis\\ebq"
"ppwue\\airoxzjjdqbvyurhaabetv"
"fxlvt"
"ql\"oqsmsvpxcg\"k"
"vqlhuec\\adw"
"qzmi\xffberakqqkk"
"tisjqff\"wf"
"yhnpudoaybwucvppj"
"xhfuf\\ehsrhsnfxcwtibd\"ubfpz"
"ihgjquzhf\""
"ff\x66dsupesrnusrtqnywoqcn\\"
"z\x77zpubbjmd"
"\"vhzlbwq\"xeimjt\\xe\x85umho\"m\"\"bmy"
"mmuvkioocmzjjysi\"mkfbec\""
"rpgghowbduw\x2fayslubajinoik\xd0hcfy"
"xrkyjqul\xdexlojgdphczp\"jfk"
"mg\x07cnr\x8b\x67xdgszmgiktpjhawho"
"kdgufhaoab"
"rlhela\"nldr"
"wzye\x87u"
"yif\x75bjhnitgoarmfgqwpmopu"
"pvlbyez\"wyy\x3dpgr"
"ezdm\"ovkruthkvdwtqwr\"ibdoawzgu"
"qubp"
"b\\kcpegcn\\zgdemgorjnk"
"gjsva\\kzaor\"\"gtpd"
"\"kt"
"rlymwlcodix"
"qqtmswowxca\"jvv"
"jni\xebwhozb"
"zhino\"kzjtmgxpi\"zzexijg"
"tyrbat\\mejgzplufxixkyg"
"lhmopxiao\x09\"p\xebl"
"xefioorxvate"
"nmcgd\x46xfujt\"w"
"\xe3wnwpat\"gtimrb"
"wpq\"xkjuw\xebbohgcagppb"
"fmvpwaca"
"mlsw"
"fdan\\\x9e"
"\"f\"fmdlzc"
"nyuj\\jnnfzdnrqmhvjrahlvzl"
"zn\"f\xcfsshcdaukkimfwk"
"uayugezzo\\\"e\"blnrgjaupqhik"
"efd\"apkndelkuvfvwyyatyttkehc"
"ufxq\\\"m\"bwkh\x93kapbqrvxxzbzp\\"
"fgypsbgjak\x79qblbeidavqtddfacq\\i\"h"
"kcfgpiysdxlgejjvgndb\\dovfpqodw"
"\"onpqnssmighipuqgwx\"nrokzgvg"
"vhjrrhfrba\"jebdanzsrdusut\\wbs"
"o\xdakymbaxakys"
"uwxhhzz\\mtmhghjn\\\\tnhzbejj"
"yd\\"
"bpgztp\\lzwpdqju\"it\x35qjhihjv"
"\\my\\b\"klnnto\\\xb3mbtsh"
"ezyvknv\"l\x2bdhhfjcvwzhjgmhwbqd\"\\"
"ftkz\"amoncbsohtaumhl\"wsodemopodq"
"ifv"
"dmzfxvzq"
"sped\"bvmf\"mmevl\"zydannpfny"
"fjxcjwlv\"pnqyrzatsjwsqfidb"
"muc\xfdqouwwnmuixru\\zlhjintplvtee"
"mraqgvmj"
"njopq\"ftcsryo"
"enoh\"n"
"t\"ntjhjc\"nzqh\xf7dcohhlsja\x7dtr"
"flbqcmcoun"
"dxkiysrn\\dyuqoaig"
"nehkzi\"h\"syktzfufotng\xdafqo"
"dzkjg\\hqjk\\\"zfegssjhn"
"sadlsjv"
"vmfnrdb\""
"ac\\bdp\"n"
"qt\x89h"
"lsndeugwvijwde\\vjapbm\\k\\nljuva"
"twpmltdzyynqt\\z\\tnund\x64hm"
"hpcyata\"ocylbkzdnhujh"
"hskzq\"knntuhscex\"q\\y\\vqj\x3an"
"eekwyufvji\\mqgeroekxeyrmymq"
"hl\"durthetvri\xebw\\jxu\"rcmiuy"
"\"fxdnmvnftxwesmvvq\"sjnf\xaabpg\"iary"
"\"\"nksqso"
"ruq\xbezugge\"d\"hwvoxmy\"iawikddxn\"x"
"rxxnlfay"
"stcu\"mv\xabcqts\\fasff"
"yrnvwfkfuzuoysfdzl\x02bk"
"qbdsmlwdbfknivtwijbwtatqfe"
"\"erqh\\csjph"
"ikfv"
"\xd2cuhowmtsxepzsivsvnvsb"
"vj"
"d"
"\\g"
"porvg\x62qghorthnc\"\\"
"tiks\\kr\"\x0fuejvuxzswnwdjscrk"
"xmgfel\"atma\\zaxmlgfjx\"ajmqf"
"oz\\rnxwljc\\\"umhymtwh"
"wlsxxhm\x7fqx\\gjoyrvccfiner\\qloluqv"
"k\\ieq"
"xidjj\"ksnlgnwxlddf\\s\\kuuleb"
"wjpnzgprzv\\maub\x0cj"
"r"
"y"
"\"yecqiei\"ire\\jdhlnnlde\xc5u"
"drvdiycqib"
"egnrbefezcrhgldrtb"
"plqodxv\\zm\"uodwjdocri\x55ucaezutm"
"f\"wexcw\x02ekewx\"alyzn"
"pqajwuk\\\\oatkfqdyspnrupo"
"rkczj\"fzntabpnygrhamk\\km\x68xfkmr"
"wejam\xbac\x37kns"
"qqmlwjk\"gh"
"fdcjsxlgx"
"\\cxvxy\"kb\"\"unubvrsq\\y\\awfhbmarj\\"
"geunceaqr"
"tpkg\"svvngk\\sizlsyaqwf"
"\"pa\\x\x18od\\emgje\\"
"ffiizogjjptubzqfuh\"cctieqcdh"
"yikhiyyrpgglpos"
"h\\"
"jotqojodcv"
"ervsz\x87ade\"fevq\\tcqowt"
"\\y\"fgrxtppkcseeg\\onxjarx\\hyhfn\x5fi"
"kxndlabn\\wwumctuzdcfiitrbnn"
"eoosynwhwm"
"\"c\x04"
"ny\xf6vuwlec"
"ubgxxcvnltzaucrzg\\xcez"
"pnocjvo\\yt"
"fcabrtqog\"a\"zj"
"o\\bha\\mzxmrfltnflv\xea"
"tbfvzwhexsdxjmxejwqqngzixcx"
"wdptrakok\"rgymturdmwfiwu"
"reffmj"
"lqm"
"\\oc"
"p\""
"ygkdnhcuehlx"
"vsqmv\"bqay\"olimtkewedzm"
"isos\x6azbnkojhxoopzetbj\xe1yd"
"yo\\pgayjcyhshztnbdv"
"fg\"h"
"vcmcojolfcf\\\\oxveua"
"w\"vyszhbrr\"jpeddpnrjlca\x69bdbopd\\z"
"jikeqv"
"\"dkjdfrtj"
"is"
"hgzx"
"z\""
"woubquq\\ag\""
"xvclriqa\xe6ltt"
"tfxinifmd"
"mvywzf\"jz"
"vlle"
"c\"rf\"wynhye\x25vccvb\""
"zvuxm"
"\xf2\"jdstiwqer\"h"
"kyogyogcknbzv\x9f\\\\e"
"kspodj\"edpeqgypc"
"oh\\x\\h"
"julb"
"bmcfkidxyilgoy\\xmu\"ig\\qg"
"veqww\"ea"
"fkdbemtgtkpqisrwlxutllxc\"mbelhs"
"e"
"ecn\x50ooprbstnq"
"\"\xe8\"ec\xeah\"qo\\g\"iuqxy\"e\"y\xe7xk\xc6d"
"lwj\"aftrcqj"
"jduij\x97zk\"rftjrixzgscxxllpqx\"bwwb"
"fqcditz"
"f\x19azclj\"rsvaokgvty\"aeq"
"erse\x9etmzhlmhy\x67yftoti"
"lsdw\xb3dmiy\\od"
"x\x6fxbljsjdgd\xaau"
"hjg\\w\"\x78uoqbsdikbjxpip\"w\"jnhzec"
"gk"
"\\zrs\\syur""#};

#[test]
fn test_a() {
    let result = part_a();
    assert_eq!(result, 1342);
}

#[test]
fn test_b() {
    let result = part_b();
    assert_eq!(result, 2074);
}
