use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::i64;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use image::{Delay, Frame, GenericImage, ImageBuffer, Rgba};
use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat::Infinite;

const BASE: i64 = 62;
const CHARACTERS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn main() {
    let data = "1=32x32+2060G0P0R031516181B1K1O1Q1V193F3N3R3U374H4J4P4T4152545C5K5N5O5Q5R5A6E6H6O6P6R6U687C7D7J7L7186898E8I8M81929395999D9O9Q9MANAVA1B5BBBCBLBNBOBQB0C3CACCCGCMCKDMDSDVD3E5EEEPERE1FAFJFOFRFTF6GFG3H6H9HFHHHSHBIJIHJMJNJOJPJVJAKKKQKTKUKBLHLQLRL8MGMMMOMSMTM8NENQNUN5O7O9OCOIOMO1P6P7PBPIPVP0Q2Q3QAQKQ0RCRGR1S3S9SMSQS4TBTCTGTJTMTNTSTVT1U8UHULURU6V9VBVMVNV+000;001;3421;253;0611;078;3711;459;443;433;558;542;529;421;415;514;504;0028;001;0421;054;5320;1146;111;4521;552;358;363;462;2617;272;159;055;043;4926;4A4;5A5;594;583;684;6A7;6B5;5C10;5D5;0036;001;1419;242;047;2514;354;268;362;3B27;4B5;5B3;6A5;4C18;3C1;2C4;2D6;0D21;0E2;0F3;1G6;0G6;0H7;1H4;3H9;4H3;5H4;6G6;7G3;6H7;7H3;0I27;0P24;1P2;2P4;1Q5;4P18;5P4;6P4;6Q3;7Q2;AQ25;8Q23;8R6;7R4;6S6;7S2;8S4;BS17;DT10;CT9;CU13;DU1;BU8;4T28;2T8;0T7;1U10;0U2;0V8;3U21;4U2;5U4;5T2;4V9;2V9;6V17;9U28;8U3;7V9;AV26;BV10;CV4;GS20;GT5;FT2;GU11;FU9;FV4;GV1;HS18;HR5;HT7;IT2;IS1;JU17;KU3;HV13;IV3;LQ24;HO27;FN12;GN2;HM11;HL2;HJ8;HI3;HG7;IG15;IF1;IH9;JG6;JF1;JE4;IE2;JD17;ID3;HD4;IK34;IL2;IM4;JM5;JL2;KH17;LH4;MH4;NI6;NJ3;NK4;NH13;NG3;KG9;KE11;KD12;LD4;ND7;OD4;OC9;NC2;PC8;QD4;QC5;QB5;OB6;MC7;LC5;MB10;LB3;MA9;LA1;NA8;R918;Q92;R812;Q82;P84;Q76;S77;S82;T614;O927;KC21;JB12;KA6;K94;L91;M94;M812;L84;N810;00296;001;1419;045;052;155;3411;547;532;524;4516;353;254;264;362;464;0616;076;175;086;288;273;373;384;487;396;495;579;674;6318;623;614;505;605;704;9622;987;894;7A9;8A3;7B11;A817;A73;B87;B74;B63;A65;A51;0030;000;3216;313;337;425;415;405;501;515;524;534;446;343;244;6329;644;542;6514;552;3617;374;2520;1613;0411;0615;074;185;194;292;393;0813;094;3A17;4A2;3B7;4B3;2B7;3D12;2D4;4E8;3E4;2E4;2F16;3F1;4F4;7939;685;776;762;754;854;861;8B21;8C3;8F11;9A31;991;AA11;A613;A53;949;848;737;832;934;924;822;724;609;718;813;709;803;904;A16;A21;A33;A013;B110;B04;B312;B43;B53;C512;C37;D319;0028;001;3317;346;327;4522;554;6612;654;644;446;428;414;306;0030;001;3214;424;2415;267;274;0048;001;3420;254;2713;284;3517;0025;001;1215;039;131;234;5326;633;622;7011;8110;801;8312;844;741;4424;4510;352;0555;0421;6C40;5D3;3E12;4E4;5E4;6E4;6D2;4F12;5F3;6F4;4H18;4I4;3G11;5J22;4J3;6K12;6M8;6N4;5M6;5N3;5L6;5K4;4L12;3K9;3J3;3I4;2I5;2J2;2K4;2H12;0039;001;2318;223;247;343;332;2015;306;404;606;807;904;9412;954;865;874;786;884;4816;384;283;268;157;0026;001;4629;3717;473;574;563;556;678;663;6313;6010;702;804;B521;D59;0025;001;1521;5417;528;514;404;506;6116;6310;644;653;4619;561;369;373;473;574;0046;001;1031;101;6022;807;918;903;A210;B23;938;A34;B34;948;7510;765;865;A66;E516;D119;E12;C011;D02;E04;F418;F53;D69;E710;E88;F84;D87;E97;F92;G95;G82;G74;H86;H93;I97;I75;J79;I413;I34;H32;0140;011;2212;3519;454;465;363;263;252;145;043;056;4721;572;2818;294;398;381;4911;595;581;685;672;664;653;756;763;774;784;695;8628;874;4A31;3A1;0A11;0B7;0C10;1C4;0D7;3C12;4C3;5B4;5C10;6B10;6A3;7A9;0032;104;0024;000;0524;062;162;264;363;373;272;1A21;2A2;3A4;3B4;2B2;0E21;1G8;0F10;0G3;0H4;1H2;2H4;3H3;4H4;4I9;3I4;1I9;2J14;3J1;4J4;2K13;0J9;0K10;1L7;2L1;3L4;2M7;4M28;5L5;6K13;6L2;4P18;9P22;9O2;9N4;AO11;AP3;AQ4;8Q6;8R3;8T10;9U23;AV10;9V3;7T16;6T15;6U2;6S7;5S4;3V15;0U22;0V11;1V1;0P26;2P8;7Q40;DV28;EV13;EU4;DQ18;DP4;EQ7;EP1;CO10;BN10;AM7;9L5;AL9;AK2;BM10;BL3;EK25;FN18;GO12;GN2;GM4;FM3;FL9;GL4;HO31;FR20;0029;001;0029;001;1316;0325;044;246;1510;051;2515;354;366;261;164;4520;443;545;554;7525;762;8412;941;B48;D47;D510;E52;E69;C68;C73;D86;E83;F426;F124;F01;G110;G02;F210;G22;G37;H33;G46;H43;G716;G86;I88;J84;K83;L76;L62;M714;M64;M53;N68;N73;L89;J99;K94;L94;M95;0035;000;5322;542;8122;712;919;0034;001;3018;9221;A36;B34;C33;D34;D25;C22;B24;9015;E125;D06;E213;E09;F421;F33;G48;G33;H58;H38;G25;H27;H114;G13;I18;H04;F08;F738;F84;E82;C910;B95;A94;BA13;CA1;FB16;DC8;EC2;IA18;GF24;IF6;JF4;JA19;KC15;LC5;MB23;MC3;NC5;NB4;ND8;OD3;OC1;OB4;MA8;LA3;NA7;N93;M93;L94;KA4;K910;H820;G910;G813;I719;K76;L74;M72;O914;O81;N717;M64;L64;N615;O64;O71;P711;P64;P89;PA11;PB13;QB4;QA1;Q94;QC13;RC5;RB1;R98;M525;L54;K52;M411;L43;J512;J44;K310;L34;M34;L26;K24;M210;N418;N24;N14;M12;L14;K05;L09;M04;Q736;Q63;Q54;P53;O48;O34;P311;P24;O23;P19;R839;S84;0035;001;6327;623;647;4928;4A1;3A3;1A9;1B9;0B9;0C4;5B20;6B3;7C9;0021;001;4118;307;408;519;503;6210;724;823;835;734;4314;5410;554;4511;462;0035;001;3217;2413;254;3619;353;344;0030;001;3221;3315;447;434;424;2126;A131;B25;C23;C17;C740;B77;B82;C83;A810;A91;B92;C94;8918;7A6;7B4;8B7;8A2;9A5;AA4;9B14;0034;001;5426;457;5323;2033;201;0026;001;2316;332;137;246;343;148;044;057;3510;438;425;448;3620;463;266;373;6826;667;6A17;6B4;0832;0910;0B5;0C4;3D16;4D2;3E7;4E4;2F14;2H5;1G11;1H2;1I6;0L26;0M1;0N5;0I23;0H9;0F20;1P34;1Q3;2Q1;0036;001;4422;544;3412;7416;844;6512;663;5611;3510;244;254;0035;001;4321;546;554;4512;448;346;354;3634;A239;A13;904;B011;B110;C13;C01;B313;C33;D35;I035;I18;0023;001;5220;623;4310;444;335;344;237;243;1212;0027;101;204;0024;001;127;236;245;145;133;035;044;6322;642;9725;999;AA5;BB13;AB4;8821;4620;355;254;266;365;2711;374;473;1713;059;065;075;083;5820;6910;8B15;AD18;AC3;BF39;BG7;CG6;CF1;CE4;BE1;DH14;EH5;FH2;GJ12;GI2;GK8;GN14;FO7;GO3;EP9;AM19;AN2;9M9;9N4;5L33;5M2;4M4;1M16;1N15;0N3;0O7;2O7;3P10;2Q10;3Q3;4Q4;2R10;3R2;4R4;3S11;4S4;3T8;4T5;2T7;1S5;1T7;1U5;2U2;4V22;5R20;5Q3;5P3;6Q13;6P2;7N22;7R18;8Q5;8P4;8R12;6S14;5T6;6T10;7T4;7S1;6U12;7U1;5V13;7V16;8T18;8U2;8N45;7M18;AP21;HO32;HN2;HM4;IO9;IN1;GH26;HH3;JB25;IB2;HB4;HC3;GC22;EC31;FD10;7980;7A2;8A36;9A18;CC20;BD18;CD4;DD4;DE24;DG20;EF32;FG21;GG5;FF12;GF2;0071;001;2417;254;2622;0723;082;5522;654;545;447;649;7512;745;841;944;951;0021;001;1523;2511;264;163;064;074;279;374;5613;678;785;794;7B8;6C7;7C2;5D11;5E4;4E2;1E12;2F12;1F3;0E5;0F6;4G16;5G4;4H9;5H5;3H8;4I9;5I1;3J11;6F23;7G8;7F2;7E4;8E4;8F2;8G4;9E16;9C6;9B4;899;884;989;A911;A82;A74;B89;B75;B64;A63;A45;8211;722;9113;A28;A12;A05;B113;B01;C214;B35;B930;AD18;BD1;AE7;AF5;BF13;BG1;AG1;9G5;BH11;CH4;CG1;CF4;CE3;9I21;9J5;1036;101;1323;233;335;342;5222;0244;031;045;2211;4011;A223;A07;B313;B21;B15;B03;C05;C26;C33;0025;001;1417;0416;054;038;1721;272;374;3813;482;288;079;082;0910;194;0A8;1A3;2A4;292;497;594;582;574;4A15;5A3;3A7;3B4;5B5;0B20;2G22;3G4;3H5;2H3;4F25;5F4;5E3;5D4;4G15;5G2;5H8;4I4;3K8;4K3;3L11;1L18;1M1;0K11;4N19;5J22;5I2;6853;674;664;6411;744;5313;617;714;705;603;504;8016;9010;A08;A14;9428;969;874;884;699;792;7A9;6B7;6C4;9725;A74;A57;A410;A34;B48;B33;B24;A24;C414;C54;D58;D45;D34;E412;E26;D22;C22;E110;D11;C15;B06;D015;E04;F519;F42;F34;G512;G44;G34;G23;F22;G16;F06;G04;H04;H413;H53;H77;H84;HA7;IA3;JA14;LA8;MA4;M92;O617;O88;P514;P023;N938;NA0;8A63;8B3;7B2;9A9;9B3;BA40;DA7;EA4;DB10;EB1;FB4;EC6;CB8;CD19;DE10;CE4;8D18;7E5;6D13;6F18;6G2;7I10;6I3;6J30;7K17;6K12;6L2;5L10;5M4;6M2;7M4;6N9;7N3;CL31;DL3;EK5;FK4;EL11;FL2;GJ10;HJ6;HI4;HG8;IH27;II13;JI2;KJ12;KI2;KH5;JJ8;JK13;JL4;KL11;KM4;LM7;LL2;LK4;ML10;MK4;MH16;NH4;NG12;OG5;QI17;RI4;SG9;TH9;TI4;SK18;TK1;RK7;SL5;TL5;RL14;QL4;NM18;PM7;QM4;TO23;TP3;TQ4;UQ5;UP2;TR18;US12;VR5;VS11;UT4;TT4;TU7;UU4;VU4;OV31;NV4;MU11;MV2;LU11;LV2;MR22;MQ3;MN15;MP9;LN10;LO3;KN9;KO1;KP17;LP3;JQ11;JR4;KR10;JS15;KS6;LS14;IS19;FP19;FQ3;GS9;FS3;HT15;IT1;JT4;HU10;JU7;KV17;LT21;EO35;DN6;DO3;CN12;CO2;BM23;BP12;CP2;DP4;DQ15;EQ1;CQ10;DR10;CR2;AQ11;AP1;AO5;BR15;CS10;BS2;BV28;CV2;DT10;ES14;ET4;DU11;8Q27;7Q3;9P14;8P1;7P4;8O9;7O2;9N12;8N2;9M26;6Q27;6S6;6T4;6V7;5U8;5V2;4V12;4U6;5T6;5S4;4S10;4O43;3O3;4R40;2O35;0O6;1P10;2P3;3P3;2Q29;3Q1;4Q4;5P28;2R25;1R23;1S4;2S3;3S4;3T9;2T2;1T4;3V22;2V12;0V6;0Q19;UN103;UI28;UJ3;VH12;VI4;VJ4;VL14;UM8;SC38;QC7;PC4;ND8;JB20;KB5;GA47;MB23;NB3;NC4;OB14;OA2;P861;P93;Q812;Q72;Q99;R92;R84;R73;R322;R214;S22;Q117;T326;U47;U33;U24;T22;V418;T116;R16;S011;T02;U03;V18;V02;V726;V84;T911;U93;TA11;UA4;SA9;VA48+3313P;0516P;2718P;4046P;0354P;0610P;4313P;5470P;2534P;1412P;067P;4821P;3A5P;5B32P;7D28P;0341P;0522P;153P;345P;0722P;2B11P;5A13P;7A7P;0C45P;1F10P;2H28P;5G10P;2I36P;1I4P;0O22P;3P32P;6O10P;8Q17P;9Q11P;BQ5P;6R17P;8Q8R;BR33P;CT8P;CT17R;BT3P;6T38P;3T12P;1T7P;2U70P;6U16P;AU26P;7U10P;8V17P;9V9P;DV22P;ET8P;EU21P;GR38P;HU30P;IU3P;LS33P;LR3P;LP9P;HP21P;EN13P;HN9P;HK14P;HH10P;HF7P;HE3P;GE3P;JH69P;II41P;MI14P;KF42P;MD27P;PD9P;NB40P;OA45P;S912P;P98P;T831P;T78P;S64P;JC51P;KB6P;N919P;O829P;13306P;2426P;446P;1654P;4743P;2910P;6431P;5111P;9532P;976P;998P;794P;23104P;3025P;4318P;6135P;624P;3533P;253P;2530R;145P;153P;177P;0521P;4950P;2A4P;2C26P;4D7P;6976P;6712P;8926P;8A8P;8D8P;8E4P;A758P;7413P;9176P;B245P;A47P;C420P;C28P;D59P;D43P;2446P;2527P;353P;5611P;4330P;317P;2248P;2315P;2511P;2811P;3417P;2445P;456P;2610P;3621P;4510R;2232P;029P;3328P;434P;6014P;8225P;4419P;543P;4411R;554P;5620P;159P;144P;054P;0527R;4D48P;6B8P;3D13P;3F7P;4G49P;6J35P;6L21P;4K36P;4M6P;3H36P;2G6P;0G11P;0439P;143P;3122P;212P;5019P;706P;9318P;857P;6814P;5818P;2711P;259P;2345P;448P;453P;5412P;6444P;629P;614P;9013P;A513P;C58P;D45P;D77P;1435P;042P;5521P;535P;3013P;6219P;604P;7470P;734P;5077P;706P;9224P;7435P;9617P;B66P;B53P;C53P;D54P;C122P;F759P;D74P;H737P;I537P;F316P;F27P;2457P;0327P;2720P;373P;565P;4844P;8565P;842P;889P;8915P;1A22P;1B9P;2C30P;4D6P;5C5P;5C13R;6C3P;5A5P;8A16P;1575P;0935P;192P;1E36P;1F5P;2G6P;2I47P;0I8P;1K33P;1J3P;4L58P;4N33P;4O4P;8P20P;8S45P;9T4P;7U52P;7S11P;3U33P;2U4P;2V8P;1U6P;0Q34P;1P4P;3P15P;6Q35P;BV25P;CV3P;ET23P;ES3P;ER3P;DR3P;CP6P;BO26P;AN6P;9M6P;9K8P;EN39P;EM3P;EL2P;HN77P;GP6P;HP3P;GQ9P;1282P;2412P;344P;149P;2416R;0643P;5324P;7427P;A422P;C46P;E47P;C58P;C826P;F821P;E031P;F329P;H24P;G627P;H815P;K710P;L512P;N523P;M838P;5256P;9225P;6124P;2336P;315P;7215P;823P;936P;B47P;9131P;D118P;E39P;E22P;E220R;F529P;F238P;I39P;I33P;I25P;F125P;J216P;F618P;C818P;FA37P;GA27P;HA3P;FF23P;HF4P;JC19P;JB8P;LB30P;KB3P;MD6P;H990P;G732P;H79P;J79P;M811P;N84P;OA7P;K634P;P945P;PC39P;N536P;K430P;J66P;J310P;N354P;K114P;N019P;O522P;P442P;O111P;Q429P;Q811P;S99P;R712P;5444P;5831P;486P;2A17P;0A7P;4B35P;3150P;426P;5236P;8111P;5316P;4412P;5612P;3619P;2235P;233P;2623P;163P;2264P;336P;3412P;339R;5420P;635P;2014P;9123P;A28P;C38P;D332P;A715P;9945P;794P;4494P;533P;5225P;5313R;3270P;0333P;2516P;5218P;548P;536P;457P;5628P;6913P;674P;6C28P;1825P;0A18P;4C18P;2D4P;2E23P;2G8P;2I5P;1N40P;0K21P;0J4P;0G22P;1F5P;1E4P;1O34P;0O19P;3239P;234P;332P;6432P;5532P;458P;1425P;4242P;333P;535P;656P;2339P;6616P;465P;5622P;A414P;A32P;A012P;B235P;E324P;H019P;3248P;423P;2248P;038P;2263P;0222P;6224P;8315P;969P;987P;A96P;DB14P;CB3P;7830P;685P;566P;573P;454P;1511P;1618P;3854P;484P;6B13P;6A3P;7B7P;9C9P;9B3P;AF11P;AE3P;BH43P;CH9P;FI32P;GL26P;GM3P;EO14P;AL28P;9L13P;9O10P;AO3P;5K21P;2M18P;0L6P;0M3P;2N12P;2P24P;4P8P;2S41P;0S33P;4U34P;6N49P;6M3P;6R28P;8O13P;5S24P;5U9P;6V51P;8S19P;8V9P;8M63P;II82P;HI3P;HG12P;HF4P;HD7P;KB11P;H920P;HA3P;GD28P;FC10P;DC22P;8989P;BC52P;DF70P;EG35P;ED79P;2348P;4577P;3418P;2574P;058P;258R;1719P;3512P;6611P;574P;777P;7A8P;5C10P;2E26P;1D5P;0D4P;3G36P;4I26P;3I7P;4I5R;6G30P;6E7P;9F35P;9D6P;8724P;997P;974P;A536P;A36P;6215P;929P;904P;B234P;B531P;BE48P;9H16P;AH6P;BI50P;2237P;129P;6331P;518P;504P;427P;413P;326P;2239R;A126P;B36P;A313P;B36R;1362P;157P;0312P;0316R;0611P;163P;4724P;1815P;3941P;1B61P;0G49P;3F12P;5C17P;4H23P;4J7P;1K35P;3N29P;54100P;5219P;514P;8132P;914P;9241P;934P;957P;864P;7810P;6A18P;B554P;C362P;E510P;E35P;B127P;C016P;G643P;F118P;H327P;H612P;H99P;KA24P;M814P;N82P;O59P;O75P;P140P;9995P;896P;7C30P;CA37P;FA11P;CC25P;BB15P;BC3P;EE12P;9D17P;8E4P;7D4P;6E27P;6H12P;8I5P;7J38P;7L16P;5K18P;5N20P;BL28P;DK10P;FJ10P;HH36P;HF6P;GF3P;JH26P;IJ18P;JM41P;KK6P;LH45P;OI12P;MG7P;PI13P;RG19P;TG6P;TJ21P;QK19P;MM40P;OM8P;SM15P;TM4P;SQ10P;UR32P;TS5P;TV58P;PV11P;NU13P;MT36P;MS5P;NQ7P;MO18P;JN39P;JO4P;JP6P;KQ26P;LQ3P;LR31P;HS26P;FO12P;FR10P;KU48P;KT3P;JV8P;FT40P;DM26P;CM13P;BN32P;BO2P;EP23P;ER21P;BQ10P;DS56P;AV8P;DV47P;9Q18P;9O35P;8M30P;6O32P;6P3P;6R9P;6U10P;4T48P;5R18P;5O20P;1O89P;4P38P;5Q52P;3R16P;1Q23P;3U45P;1V20P;0R17P;0P10P;TN95P;VN7P;UH27P;UL32P;VM18P;TC29P;RC7P;OC10P;NE5P;IB18P;GA40P;GA14R;LB20P;O930P;P642P;P82P;P816R;P79P;Q340P;S36P;Q27P;Q030P;T420P;U148P;S16P;R011P;V639P;U87P;S913P;V973P;UB23P".to_string();

    let option = data.split_once('=').unwrap();
    let version = option.0;

    let split: Vec<&str> = option.1.split('+').collect();

    //Version 1 requires all data to exist, empty data has to be marked with an `++` but I might not be omitted
    //Only metadata and mine data might not be empty
    if version.eq("1") {
        parse_v1(split[0], split[1], split[2], split[3])
    }
}

fn parse_v1(raw_meta: &str, raw_mine_data: &str, raw_open_data: &str, raw_flag_data: &str) {
    let metadata = parse_meta_data(raw_meta).unwrap();
    let mut game_board = parse_mine_data(raw_mine_data, &metadata).unwrap();
    let open_data = &mut parse_open_data(raw_open_data).unwrap();
    let flag_data = &mut parse_flag_data(raw_flag_data).unwrap();

    // println!("{metadata:?}\n{game_board:?}\n{open_data:?}\n{flag_data:?}");

    let mut frames = Vec::new();

    let mut test: BTreeMap<i64, Vec<ActionType>> = BTreeMap::new();
    for x in open_data.iter() {
        if let std::collections::btree_map::Entry::Vacant(e) = test.entry(x.total_time) {
            e.insert(vec![ActionType::Open]);
        } else {
            test.get_mut(&x.total_time).unwrap().push(ActionType::Open);
        }
    }

    for x in flag_data.iter() {
        if let std::collections::btree_map::Entry::Vacant(e) = test.entry(x.total_time) {
            e.insert(vec![ActionType::Flag]);
        } else {
            test.get_mut(&x.total_time).unwrap().push(ActionType::Flag);
        }
    }

    for (id, tick) in test.iter().enumerate() {
        let next_tick = test.keys().nth(id + 1);
        let duration = if let Some(next) = next_tick {
            Duration::from_millis(((next - tick.0) * 50) as u64)
        } else {
            Duration::from_secs(15)
        };
        let frame = generate_image(&mut game_board, &metadata);
        frames.push(Frame::from_parts(frame, 0, 0, Delay::from_saturating_duration(duration)));


        if tick.1.contains(&ActionType::Open) {
            open_data.iter()
                .filter(|flag| { flag.total_time.eq(tick.0) })
                .for_each(|action| {
                    game_board.open_field(action.x as usize, action.y as usize);
                });

            //Remove all elements which are less than tick.0
            open_data.retain(|open| {open.total_time.gt(tick.0)})
        } else if tick.1.contains(&ActionType::Flag) {
            flag_data.iter()
                .filter(|flag| { flag.total_time.eq(tick.0) })
                .for_each(|flag| {
                    match flag.action {
                        Action::Place => game_board.fields[flag.y as usize][flag.x as usize].field_state = FieldState::Flagged,
                        Action::Remove => game_board.fields[flag.y as usize][flag.x as usize].field_state = FieldState::Closed,
                    }
                });
            //Remove all elements which are less than tick.0
            flag_data.retain(|flag| {flag.total_time.gt(tick.0)})
        }
    }

    let mut gif_encoder = GifEncoder::new(File::create("animation.gif").unwrap());
    gif_encoder.set_repeat(Infinite).unwrap();
    let le = frames.len();
    for (i, frame) in frames.into_iter().enumerate() {
        println!("Encoding frame {} of {}", i, le);
        gif_encoder.encode_frame(frame).unwrap();
    }
}

fn generate_image(board: &mut Board, metadata: &Metadata) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let imgx = (metadata.x_size * 32) as u32;
    let imgy = (metadata.y_size * 32) as u32;

    let im = &mut image::open(Path::new(&"skin.png")).unwrap();
    let zero = im.sub_image(0, 0, 32, 32).to_image();
    let one = im.sub_image(32, 0, 32, 32).to_image();
    let two = im.sub_image(32 * 2, 0, 32, 32).to_image();
    let three = im.sub_image(32 * 3, 0, 32, 32).to_image();
    let four = im.sub_image(32 * 4, 0, 32, 32).to_image();
    let five = im.sub_image(32 * 5, 0, 32, 32).to_image();
    let six = im.sub_image(32 * 6, 0, 32, 32).to_image();
    let seven = im.sub_image(32 * 7, 0, 32, 32).to_image();
    let eight = im.sub_image(32 * 8, 0, 32, 32).to_image();
    let tnt = im.sub_image(32 * 9, 0, 32, 32).to_image();
    let closed = im.sub_image(32 * 10, 0, 32, 32).to_image();
    let flag = im.sub_image(32 * 11, 0, 32, 32).to_image();

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for x in 0..metadata.x_size as u32 {
        for y in 0..metadata.y_size as u32 {
            let field = &mut board.fields[x as usize][y as usize];
            let xx = x * 32;
            let yy = y * 32;
            if field.field_state == FieldState::Closed {
                imgbuf.copy_from(&closed, xx, yy).expect("TODO: panic message");
                continue;
            }
            if field.field_state == FieldState::Flagged {
                imgbuf.copy_from(&flag, xx, yy).expect("TODO: panic message");
                continue;
            }
            if field.mine {
                imgbuf.copy_from(&tnt, xx, yy).expect("TODO: panic message");
                continue;
            }
            match field.value {
                0 => imgbuf.copy_from(&zero, xx, yy).expect("TODO: panic message"),
                1 => imgbuf.copy_from(&one, xx, yy).expect("TODO: panic message"),
                2 => imgbuf.copy_from(&two, xx, yy).expect("TODO: panic message"),
                3 => imgbuf.copy_from(&three, xx, yy).expect("TODO: panic message"),
                4 => imgbuf.copy_from(&four, xx, yy).expect("TODO: panic message"),
                5 => imgbuf.copy_from(&five, xx, yy).expect("TODO: panic message"),
                6 => imgbuf.copy_from(&six, xx, yy).expect("TODO: panic message"),
                7 => imgbuf.copy_from(&seven, xx, yy).expect("TODO: panic message"),
                8 => imgbuf.copy_from(&eight, xx, yy).expect("TODO: panic message"),
                _ => unreachable!(),
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    // imgbuf.save("output.png").unwrap();
    imgbuf
}

fn parse_mine_data(data: &str, metadata: &Metadata) -> Result<Board, ()> {
    let mut board = Board {
        fields: vec![vec![Field::new(); metadata.y_size as usize]; metadata.x_size as usize],
        metadata: metadata.clone(),
    };

    let mines = parse_mine_locations(data).unwrap();

    for cords in mines {
        let x = cords.0;
        let y = cords.1;
        let mut field = &mut board.fields[x as usize][y as usize];
        field.mine = true;
    }

    for x in 0..metadata.x_size {
        for y in 0..metadata.y_size {
            let field = &mut board.fields[x as usize][y as usize];

            if !field.mine {
                continue;
            }

            for xd in -1..=1_i32 {
                for zd in -1..=1_i32 {
                    let xx = x + xd;
                    let yy = y + zd;
                    if xx < 0
                        || xx >= metadata.x_size
                        || yy < 0
                        || yy >= metadata.y_size
                        || (zd == 0 && xd == 0)
                    {
                        continue;
                    }

                    let checked_field = &mut board.fields[xx as usize][yy as usize];
                    if checked_field.mine {
                        continue;
                    }

                    checked_field.value += 1;
                }
            }
        }
    }

    Ok(board)
}

fn parse_mine_locations(data: &str) -> Result<Vec<(i32, i32)>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let part = raw_open_field.split_once('|').unwrap();

            return_data.push((decode(part.0) as i32, decode(part.1) as i32));
        } else {
            raw_open_field
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect::<String>())
                .for_each(|x| {
                    let mut chars = x.chars();
                    return_data.push((
                        decode(chars.next().unwrap().to_string().as_str()) as i32,
                        decode(chars.next().unwrap().to_string().as_str()) as i32,
                    ))
                });
        }
    }

    Ok(return_data)
}

fn parse_flag_data(data: &str) -> Result<Vec<FlagAction>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let mut chars = raw_open_field.chars();

            let action_type = chars.next_back().unwrap();
            let part_one = chars.as_str().split_once('|').unwrap();
            let part_two = part_one.1.split_once(':').unwrap();

            return_data.push(FlagAction {
                x: decode(part_one.0) as i32,
                y: decode(part_two.0) as i32,
                time: part_two.1.parse::<i64>().unwrap(),
                action: get_flag_type(action_type),
                total_time: return_data.iter().map(|x| { x.time }).sum(),
            });
        } else {
            let mut chars = raw_open_field.chars();
            return_data.push(FlagAction {
                x: decode(chars.next().unwrap().to_string().as_str()) as i32,
                y: decode(chars.next().unwrap().to_string().as_str()) as i32,
                action: get_flag_type(chars.next_back().unwrap()),
                time: chars.as_str().parse::<i64>().unwrap(),
                total_time: return_data.iter().map(|x| { x.time }).sum(),
            });
        }
    }

    Ok(return_data)
}

fn get_flag_type(raw_flag_type: char) -> Action {
    match raw_flag_type {
        'P' => Action::Place,
        'R' => Action::Remove,
        _ => unreachable!(),
    }
}

fn parse_open_data(data: &str) -> Result<Vec<OpenAction>, ()> {
    let mut return_data = Vec::new();

    if data.chars().count() == 0 {
        return Ok(return_data);
    }

    let raw_open_fields_data: Vec<&str> = data.split(';').collect();

    for raw_open_field in raw_open_fields_data {
        if raw_open_field.contains('|') {
            let part_one = raw_open_field.split_once('|').unwrap();
            let part_two = part_one.1.split_once(':').unwrap();

            return_data.push(OpenAction {
                x: decode(part_one.0) as i32,
                y: decode(part_two.0) as i32,
                time: part_two.1.parse::<i64>().unwrap(),
                total_time: return_data.iter().map(|x| { x.time }).sum(),
            });
        } else {
            let mut chars = raw_open_field.chars();
            return_data.push(OpenAction {
                x: decode(chars.next().unwrap().to_string().as_str()) as i32,
                y: decode(chars.next().unwrap().to_string().as_str()) as i32,
                time: chars.as_str().parse::<i64>().unwrap(),
                total_time: return_data.iter().map(|x| { x.time }).sum(),
            });
        }
    }

    Ok(return_data)
}

fn parse_meta_data(data: &str) -> Result<Metadata, ()> {
    let data_split = data.split_once('x').unwrap();
    Ok(Metadata {
        x_size: i32::from_str(data_split.0).unwrap(),
        y_size: i32::from_str(data_split.1).unwrap(),
    })
}

fn encode(number: i64) -> String {
    let mut result = String::with_capacity(1);
    let mut num = number;

    while num > 0 {
        let digit = num % BASE;
        num /= BASE;
        result.insert(0, CHARACTERS.chars().nth(digit as usize).unwrap());
    }

    result
}

fn decode(number: &str) -> i64 {
    let mut result: i64 = 0;
    let length = number.len();
    let chars: Vec<char> = CHARACTERS.chars().collect();

    for i in 0..length {
        let digit = chars
            .iter()
            .position(|&c| c == number.chars().nth(length - i - 1).unwrap())
            .unwrap() as i64;
        result += BASE.pow(i as u32) * digit;
    }

    result
}

impl Board {
    fn open_field(&mut self, x: usize, y: usize) {
        let field = &mut self.fields[y][x];

        //If flagged or already open return
        if field.field_state != FieldState::Closed {
            return;
        }

        if field.mine {
            return;
        }

        field.field_state = FieldState::Open;

        if field.value == 0 {
            for xd in -1..=1_i32 {
                for yd in -1..=1_i32 {
                    let xx = xd + x as i32;
                    let yy = yd + y as i32;
                    if xx < 0
                        || xx >= self.metadata.x_size
                        || yy < 0
                        || yy >= self.metadata.y_size
                        || xd == 0 && yd == 0
                    {
                        continue;
                    }
                    self.open_field(xx as usize, yy as usize)
                }
            }
        }
    }

    fn print(&self) {
        for x in &self.fields {
            for field in x {
                print!("{} ", Self::get_field_text(field));
            }
            println!()
        }
    }

    fn get_field_text(field: &Field) -> String {
        match field.field_state {
            FieldState::Open => field.value.to_string(),
            FieldState::Closed => "_".to_string(),
            FieldState::Flagged => "¶".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Metadata {
    x_size: i32,
    y_size: i32,
}

#[derive(Debug)]
struct FlagAction {
    x: i32,
    y: i32,
    time: i64,
    action: Action,
    total_time: i64,
}

#[derive(Debug)]
enum Action {
    Place,
    Remove,
}

#[derive(Debug)]
struct OpenAction {
    x: i32,
    y: i32,
    time: i64,
    total_time: i64,
}

#[derive(Debug)]
struct Board {
    fields: Vec<Vec<Field>>,
    metadata: Metadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub value: u8,
    pub field_state: FieldState,
    pub mine: bool,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FieldState {
    Open,
    Closed,
    Flagged,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ActionType {
    Open,
    Flag,
}

impl Field {
    pub(crate) fn new() -> Self {
        Field {
            value: 0,
            field_state: FieldState::Closed,
            mine: false,
        }
    }
}
