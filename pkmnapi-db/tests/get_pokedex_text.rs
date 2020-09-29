use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! get_pokedex_text_test {
    ($test_name: ident, $pokedex_id: expr, $pokedex_text: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokedex_text(&$pokedex_id) {
                Ok(pokedex_text) => assert_eq!(
                    pokedex_text,
                    PokedexText {
                        text: ROMString::from($pokedex_text),
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_pokedex_text_test)]

get_pokedex_text_test!(get_pokedex_text_1, 1, "A strange seed was\nplanted on its\nback at birth.¶The plant sprouts\nand grows with\nthis #MON");
get_pokedex_text_test!(get_pokedex_text_2, 2, "When the bulb on\nits back grows\nlarge, it appears¶to lose the\nability to stand\non its hind legs");
get_pokedex_text_test!(
    get_pokedex_text_3,
    3,
    "The plant blooms\nwhen it is\nabsorbing solar¶energy. It stays\non the move to\nseek sunlight"
);
get_pokedex_text_test!(get_pokedex_text_4, 4, "Obviously prefers\nhot places. When\nit rains, steam¶is said to spout\nfrom the tip of\nits tail");
get_pokedex_text_test!(
    get_pokedex_text_5,
    5,
    "When it swings\nits burning tail,\nit elevates the¶temperature to\nunbearably high\nlevels"
);
get_pokedex_text_test!(get_pokedex_text_6, 6, "Spits fire that\nis hot enough to\nmelt boulders.¶Known to cause\nforest fires\nunintentionally");
get_pokedex_text_test!(get_pokedex_text_7, 7, "After birth, its\nback swells and\nhardens into a¶shell. Powerfully\nsprays foam from\nits mouth");
get_pokedex_text_test!(get_pokedex_text_8, 8, "Often hides in\nwater to stalk\nunwary prey. For¶swimming fast, it\nmoves its ears to\nmaintain balance");
get_pokedex_text_test!(get_pokedex_text_9, 9, "A brutal #MON\nwith pressurized\nwater jets on its¶shell. They are\nused for high\nspeed tackles");
get_pokedex_text_test!(get_pokedex_text_10, 10, "Its short feet\nare tipped with\nsuction pads that¶enable it to\ntirelessly climb\nslopes and walls");
get_pokedex_text_test!(get_pokedex_text_11, 11, "This #MON is\nvulnerable to\nattack while its¶shell is soft,\nexposing its weak\nand tender body");
get_pokedex_text_test!(
    get_pokedex_text_12,
    12,
    "In battle, it\nflaps its wings\nat high speed to¶release highly\ntoxic dust into\nthe air"
);
get_pokedex_text_test!(
    get_pokedex_text_13,
    13,
    "Often found in\nforests, eating\nleaves.¶It has a sharp\nvenomous stinger\non its head"
);
get_pokedex_text_test!(get_pokedex_text_14, 14, "Almost incapable\nof moving, this\n#MON can only¶harden its shell\nto protect itself\nfrom predators");
get_pokedex_text_test!(get_pokedex_text_15, 15, "Flies at high\nspeed and attacks\nusing its large¶venomous stingers\non its forelegs\nand tail");
get_pokedex_text_test!(get_pokedex_text_16, 16, "A common sight in\nforests and woods.\nIt flaps its¶wings at ground\nlevel to kick up\nblinding sand");
get_pokedex_text_test!(get_pokedex_text_17, 17, "Very protective\nof its sprawling\nterritorial area,¶this #MON will\nfiercely peck at\nany intruder");
get_pokedex_text_test!(get_pokedex_text_18, 18, "When hunting, it\nskims the surface\nof water at high¶speed to pick off\nunwary prey such\nas MAGIKARP");
get_pokedex_text_test!(
    get_pokedex_text_19,
    19,
    "Bites anything\nwhen it attacks.\nSmall and very¶quick, it is a\ncommon sight in\nmany places"
);
get_pokedex_text_test!(get_pokedex_text_20, 20, "It uses its whis-\nkers to maintain\nits balance.¶It apparently\nslows down if\nthey are cut off");
get_pokedex_text_test!(
    get_pokedex_text_21,
    21,
    "Eats bugs in\ngrassy areas. It\nhas to flap its¶short wings at\nhigh speed to\nstay airborne"
);
get_pokedex_text_test!(get_pokedex_text_22, 22, "With its huge and\nmagnificent wings,\nit can keep aloft¶without ever\nhaving to land\nfor rest");
get_pokedex_text_test!(
    get_pokedex_text_23,
    23,
    "Moves silently\nand stealthily.\nEats the eggs of¶birds, such as\nPIDGEY and\nSPEAROW, whole"
);
get_pokedex_text_test!(
    get_pokedex_text_24,
    24,
    "It is rumored that\nthe ferocious\nwarning markings¶on its belly\ndiffer from area\nto area"
);
get_pokedex_text_test!(get_pokedex_text_25, 25, "When several of\nthese #MON\ngather, their¶electricity could\nbuild and cause\nlightning storms");
get_pokedex_text_test!(
    get_pokedex_text_26,
    26,
    "Its long tail\nserves as a\nground to protect¶itself from its\nown high voltage\npower"
);
get_pokedex_text_test!(get_pokedex_text_27, 27, "Burrows deep\nunderground in\narid locations¶far from water.\nIt only emerges\nto hunt for food");
get_pokedex_text_test!(get_pokedex_text_28, 28, "Curls up into a\nspiny ball when\nthreatened. It¶can roll while\ncurled up to\nattack or escape");
get_pokedex_text_test!(get_pokedex_text_29, 29, "Although small,\nits venomous\nbarbs render this¶#MON dangerous.\nThe female has\nsmaller horns");
get_pokedex_text_test!(
    get_pokedex_text_30,
    30,
    "The female's horn\ndevelops slowly.\nPrefers physical¶attacks such as\nclawing and\nbiting"
);
get_pokedex_text_test!(get_pokedex_text_31, 31, "Its hard scales\nprovide strong\nprotection. It¶uses its hefty\nbulk to execute\npowerful moves");
get_pokedex_text_test!(get_pokedex_text_32, 32, "Stiffens its ears\nto sense danger.\nThe larger its¶horns, the more\npowerful its\nsecreted venom");
get_pokedex_text_test!(get_pokedex_text_33, 33, "An aggressive\n#MON that is\nquick to attack.¶The horn on its\nhead secretes a\npowerful venom");
get_pokedex_text_test!(
    get_pokedex_text_34,
    34,
    "It uses its\npowerful tail in\nbattle to smash,¶constrict, then\nbreak the prey's\nbones"
);
get_pokedex_text_test!(
    get_pokedex_text_35,
    35,
    "Its magical and\ncute appeal has\nmany admirers.¶It is rare and\nfound only in\ncertain areas"
);
get_pokedex_text_test!(
    get_pokedex_text_36,
    36,
    "A timid fairy\n#MON that is\nrarely seen. It¶will run and hide\nthe moment it\nsenses people"
);
get_pokedex_text_test!(get_pokedex_text_37, 37, "At the time of\nbirth, it has\njust one tail.¶The tail splits\nfrom its tip as\nit grows older");
get_pokedex_text_test!(get_pokedex_text_38, 38, "Very smart and\nvery vengeful.\nGrabbing one of¶its many tails\ncould result in a\n1000-year curse");
get_pokedex_text_test!(get_pokedex_text_39, 39, "When its huge eyes\nlight up, it sings\na mysteriously¶soothing melody\nthat lulls its\nenemies to sleep");
get_pokedex_text_test!(get_pokedex_text_40, 40, "The body is soft\nand rubbery. When\nangered, it will¶suck in air and\ninflate itself to\nan enormous size");
get_pokedex_text_test!(get_pokedex_text_41, 41, "Forms colonies in\nperpetually dark\nplaces. Uses¶ultrasonic waves\nto identify and\napproach targets");
get_pokedex_text_test!(get_pokedex_text_42, 42, "Once it strikes,\nit will not stop\ndraining energy¶from the victim\neven if it gets\ntoo heavy to fly");
get_pokedex_text_test!(get_pokedex_text_43, 43, "During the day,\nit keeps its face\nburied in the¶ground. At night,\nit wanders around\nsowing its seeds");
get_pokedex_text_test!(get_pokedex_text_44, 44, "The fluid that\noozes from its\nmouth isn't drool.¶It is a nectar\nthat is used to\nattract prey");
get_pokedex_text_test!(get_pokedex_text_45, 45, "The larger its\npetals, the more\ntoxic pollen it¶contains. Its big\nhead is heavy and\nhard to hold up");
get_pokedex_text_test!(get_pokedex_text_46, 46, "Burrows to suck\ntree roots. The\nmushrooms on its¶back grow by draw-\ning nutrients from\nthe bug host");
get_pokedex_text_test!(get_pokedex_text_47, 47, "A host-parasite\npair in which the\nparasite mushroom¶has taken over the\nhost bug. Prefers\ndamp places");
get_pokedex_text_test!(get_pokedex_text_48, 48, "Lives in the\nshadows of tall\ntrees where it¶eats insects. It\nis attracted by\nlight at night");
get_pokedex_text_test!(get_pokedex_text_49, 49, "The dust-like\nscales covering\nits wings are¶color coded to\nindicate the kinds\nof poison it has");
get_pokedex_text_test!(get_pokedex_text_50, 50, "Lives about one\nyard underground\nwhere it feeds on¶plant roots. It\nsometimes appears\nabove ground");
get_pokedex_text_test!(get_pokedex_text_51, 51, "A team of DIGLETT\ntriplets.\nIt triggers huge¶earthquakes by\nburrowing 60 miles\nunderground");
get_pokedex_text_test!(get_pokedex_text_52, 52, "Adores circular\nobjects. Wanders\nthe streets on a¶nightly basis to\nlook for dropped\nloose change");
get_pokedex_text_test!(get_pokedex_text_53, 53, "Although its fur\nhas many admirers,\nit is tough to¶raise as a pet\nbecause of its\nfickle meanness");
get_pokedex_text_test!(get_pokedex_text_54, 54, "While lulling its\nenemies with its\nvacant look, this¶wily #MON will\nuse psychokinetic\npowers");
get_pokedex_text_test!(get_pokedex_text_55, 55, "Often seen swim-\nming elegantly by\nlake shores. It¶is often mistaken\nfor the Japanese\nmonster, Kappa");
get_pokedex_text_test!(get_pokedex_text_56, 56, "Extremely quick to\nanger. It could\nbe docile one¶moment then\nthrashing away\nthe next instant");
get_pokedex_text_test!(get_pokedex_text_57, 57, "Always furious\nand tenacious to\nboot. It will not¶abandon chasing\nits quarry until\nit is caught");
get_pokedex_text_test!(
    get_pokedex_text_58,
    58,
    "Very protective\nof its territory.\nIt will bark and¶bite to repel\nintruders from\nits space"
);
get_pokedex_text_test!(get_pokedex_text_59, 59, "A #MON that\nhas been admired\nsince the past¶for its beauty.\nIt runs agilely\nas if on wings");
get_pokedex_text_test!(get_pokedex_text_60, 60, "Its newly grown\nlegs prevent it\nfrom running. It¶appears to prefer\nswimming than\ntrying to stand");
get_pokedex_text_test!(
    get_pokedex_text_61,
    61,
    "Capable of living\nin or out of\nwater. When out¶of water, it\nsweats to keep\nits body slimy"
);
get_pokedex_text_test!(get_pokedex_text_62, 62, "An adept swimmer\nat both the front\ncrawl and breast¶stroke. Easily\novertakes the best\nhuman swimmers");
get_pokedex_text_test!(
    get_pokedex_text_63,
    63,
    "Using its ability\nto read minds, it\nwill identify¶impending danger\nand TELEPORT to\nsafety"
);
get_pokedex_text_test!(
    get_pokedex_text_64,
    64,
    "It emits special\nalpha waves from\nits body that¶induce headaches\njust by being\nclose by"
);
get_pokedex_text_test!(get_pokedex_text_65, 65, "Its brain can out-\nperform a super-\ncomputer.¶Its intelligence\nquotient is said\nto be 5,000");
get_pokedex_text_test!(get_pokedex_text_66, 66, "Loves to build\nits muscles.\nIt trains in all¶styles of martial\narts to become\neven stronger");
get_pokedex_text_test!(get_pokedex_text_67, 67, "Its muscular body\nis so powerful, it\nmust wear a power¶save belt to be\nable to regulate\nits motions");
get_pokedex_text_test!(get_pokedex_text_68, 68, "Using its heavy\nmuscles, it throws\npowerful punches¶that can send the\nvictim clear over\nthe horizon");
get_pokedex_text_test!(get_pokedex_text_69, 69, "A carnivorous\n#MON that traps\nand eats bugs.¶It uses its root\nfeet to soak up\nneeded moisture");
get_pokedex_text_test!(get_pokedex_text_70, 70, "It spits out\nPOISONPOWDER to\nimmobilize the¶enemy and then\nfinishes it with\na spray of ACID");
get_pokedex_text_test!(get_pokedex_text_71, 71, "Said to live in\nhuge colonies\ndeep in jungles,¶although no one\nhas ever returned\nfrom there");
get_pokedex_text_test!(get_pokedex_text_72, 72, "Drifts in shallow\nseas. Anglers who\nhook them by¶accident are\noften punished by\nits stinging acid");
get_pokedex_text_test!(get_pokedex_text_73, 73, "The tentacles are\nnormally kept\nshort. On hunts,¶they are extended\nto ensnare and\nimmobilize prey");
get_pokedex_text_test!(get_pokedex_text_74, 74, "Found in fields\nand mountains.\nMistaking them¶for boulders,\npeople often step\nor trip on them");
get_pokedex_text_test!(get_pokedex_text_75, 75, "Rolls down slopes\nto move. It rolls\nover any obstacle¶without slowing\nor changing its\ndirection");
get_pokedex_text_test!(get_pokedex_text_76, 76, "Its boulder-like\nbody is extremely\nhard. It can¶easily withstand\ndynamite blasts\nwithout damage");
get_pokedex_text_test!(get_pokedex_text_77, 77, "Its hooves are 10\ntimes harder than\ndiamonds. It can¶trample anything\ncompletely flat\nin little time");
get_pokedex_text_test!(
    get_pokedex_text_78,
    78,
    "Very competitive,\nthis #MON will\nchase anything¶that moves fast\nin the hopes of\nracing it"
);
get_pokedex_text_test!(
    get_pokedex_text_79,
    79,
    "Incredibly slow\nand dopey. It\ntakes 5 seconds¶for it to feel\npain when under\nattack"
);
get_pokedex_text_test!(get_pokedex_text_80, 80, "The SHELLDER that\nis latched onto\nSLOWPOKE's tail¶is said to feed\non the host's left\nover scraps");
get_pokedex_text_test!(get_pokedex_text_81, 81, "Uses anti-gravity\nto stay suspended.\nAppears without¶warning and uses\nTHUNDER WAVE and\nsimilar moves");
get_pokedex_text_test!(get_pokedex_text_82, 82, "Formed by several\nMAGNEMITEs linked\ntogether. They¶frequently appear\nwhen sunspots\nflare up");
get_pokedex_text_test!(
    get_pokedex_text_83,
    83,
    "The sprig of\ngreen onions it\nholds is its¶weapon. It is\nused much like a\nmetal sword"
);
get_pokedex_text_test!(get_pokedex_text_84, 84, "A bird that makes\nup for its poor\nflying with its¶fast foot speed.\nLeaves giant\nfootprints");
get_pokedex_text_test!(get_pokedex_text_85, 85, "Uses its three\nbrains to execute\ncomplex plans.¶While two heads\nsleep, one head\nstays awake");
get_pokedex_text_test!(
    get_pokedex_text_86,
    86,
    "The protruding\nhorn on its head\nis very hard.¶It is used for\nbashing through\nthick ice"
);
get_pokedex_text_test!(get_pokedex_text_87, 87, "Stores thermal\nenergy in its\nbody. Swims at a¶steady 8 knots\neven in intensely\ncold waters");
get_pokedex_text_test!(get_pokedex_text_88, 88, "Appears in filthy\nareas. Thrives by\nsucking up¶polluted sludge\nthat is pumped\nout of factories");
get_pokedex_text_test!(get_pokedex_text_89, 89, "Thickly covered\nwith a filthy,\nvile sludge. It¶is so toxic, even\nits footprints\ncontain poison");
get_pokedex_text_test!(
    get_pokedex_text_90,
    90,
    "Its hard shell\nrepels any kind\nof attack.¶It is vulnerable\nonly when its\nshell is open"
);
get_pokedex_text_test!(
    get_pokedex_text_91,
    91,
    "When attacked, it\nlaunches its\nhorns in quick¶volleys. Its\ninnards have\nnever been seen"
);
get_pokedex_text_test!(
    get_pokedex_text_92,
    92,
    "Almost invisible,\nthis gaseous\n#MON cloaks¶the target and\nputs it to sleep\nwithout notice"
);
get_pokedex_text_test!(get_pokedex_text_93, 93, "Because of its\nability to slip\nthrough block¶walls, it is said\nto be from an-\nother dimension");
get_pokedex_text_test!(get_pokedex_text_94, 94, "Under a full moon,\nthis #MON\nlikes to mimic¶the shadows of\npeople and laugh\nat their fright");
get_pokedex_text_test!(get_pokedex_text_95, 95, "As it grows, the\nstone portions of\nits body harden¶to become similar\nto a diamond, but\ncolored black");
get_pokedex_text_test!(get_pokedex_text_96, 96, "Puts enemies to\nsleep then eats\ntheir dreams.¶Occasionally gets\nsick from eating\nbad dreams");
get_pokedex_text_test!(get_pokedex_text_97, 97, "When it locks eyes\nwith an enemy, it\nwill use a mix of¶PSI moves such as\nHYPNOSIS and\nCONFUSION");
get_pokedex_text_test!(get_pokedex_text_98, 98, "Its pincers are\nnot only powerful\nweapons, they are¶used for balance\nwhen walking\nsideways");
get_pokedex_text_test!(get_pokedex_text_99, 99, "The large pincer\nhas 10000 hp of\ncrushing power.¶However, its huge\nsize makes it\nunwieldy to use");
get_pokedex_text_test!(
    get_pokedex_text_100,
    100,
    "Usually found in\npower plants.\nEasily mistaken¶for a # BALL,\nthey have zapped\nmany people"
);
get_pokedex_text_test!(get_pokedex_text_101, 101, "It stores electric\nenergy under very\nhigh pressure.¶It often explodes\nwith little or no\nprovocation");
get_pokedex_text_test!(
    get_pokedex_text_102,
    102,
    "Often mistaken\nfor eggs.\nWhen disturbed,¶they quickly\ngather and attack\nin swarms"
);
get_pokedex_text_test!(get_pokedex_text_103, 103, "Legend has it that\non rare occasions,\none of its heads¶will drop off and\ncontinue on as an\nEXEGGCUTE");
get_pokedex_text_test!(
    get_pokedex_text_104,
    104,
    "Because it never\nremoves its skull\nhelmet, no one¶has ever seen\nthis #MON's\nreal face"
);
get_pokedex_text_test!(get_pokedex_text_105, 105, "The bone it holds\nis its key weapon.\nIt throws the¶bone skillfully\nlike a boomerang\nto KO targets");
get_pokedex_text_test!(get_pokedex_text_106, 106, "When in a hurry,\nits legs lengthen\nprogressively.¶It runs smoothly\nwith extra long,\nloping strides");
get_pokedex_text_test!(get_pokedex_text_107, 107, "While apparently\ndoing nothing, it\nfires punches in¶lightning fast\nvolleys that are\nimpossible to see");
get_pokedex_text_test!(get_pokedex_text_108, 108, "Its tongue can be\nextended like a\nchameleon's. It¶leaves a tingling\nsensation when it\nlicks enemies");
get_pokedex_text_test!(get_pokedex_text_109, 109, "Because it stores\nseveral kinds of\ntoxic gases in¶its body, it is\nprone to exploding\nwithout warning");
get_pokedex_text_test!(
    get_pokedex_text_110,
    110,
    "Where two kinds\nof poison gases\nmeet, 2 KOFFINGs¶can fuse into a\nWEEZING over many\nyears"
);
get_pokedex_text_test!(get_pokedex_text_111, 111, "Its massive bones\nare 1000 times\nharder than human¶bones. It can\neasily knock a\ntrailer flying");
get_pokedex_text_test!(
    get_pokedex_text_112,
    112,
    "Protected by an\narmor-like hide,\nit is capable of¶living in molten\nlava of 3,600\ndegrees"
);
get_pokedex_text_test!(
    get_pokedex_text_113,
    113,
    "A rare and elusive\n#MON that is\nsaid to bring¶happiness to those\nwho manage to get\nit"
);
get_pokedex_text_test!(get_pokedex_text_114, 114, "The whole body is\nswathed with wide\nvines that are¶similar to sea-\nweed. Its vines\nshake as it walks");
get_pokedex_text_test!(
    get_pokedex_text_115,
    115,
    "The infant rarely\nventures out of\nits mother's¶protective pouch\nuntil it is 3\nyears old"
);
get_pokedex_text_test!(get_pokedex_text_116, 116, "Known to shoot\ndown flying bugs\nwith precision¶blasts of ink\nfrom the surface\nof the water");
get_pokedex_text_test!(get_pokedex_text_117, 117, "Capable of swim-\nming backwards by\nrapidly flapping¶its wing-like\npectoral fins and\nstout tail");
get_pokedex_text_test!(get_pokedex_text_118, 118, "Its tail fin\nbillows like an\nelegant ballroom¶dress, giving it\nthe nickname of\nthe Water Queen");
get_pokedex_text_test!(get_pokedex_text_119, 119, "In the autumn\nspawning season,\nthey can be seen¶swimming power-\nfully up rivers\nand creeks");
get_pokedex_text_test!(
    get_pokedex_text_120,
    120,
    "An enigmatic\n#MON that can\neffortlessly¶regenerate any\nappendage it\nloses in battle"
);
get_pokedex_text_test!(get_pokedex_text_121, 121, "Its central core\nglows with the\nseven colors of¶the rainbow. Some\npeople value the\ncore as a gem");
get_pokedex_text_test!(
    get_pokedex_text_122,
    122,
    "If interrupted\nwhile it is\nmiming, it will¶slap around the\noffender with its\nbroad hands"
);
get_pokedex_text_test!(
    get_pokedex_text_123,
    123,
    "With ninja-like\nagility and speed,\nit can create the¶illusion that\nthere is more\nthan one"
);
get_pokedex_text_test!(get_pokedex_text_124, 124, "It seductively\nwiggles its hips\nas it walks. It¶can cause people\nto dance in\nunison with it");
get_pokedex_text_test!(get_pokedex_text_125, 125, "Normally found\nnear power plants,\nthey can wander¶away and cause\nmajor blackouts\nin cities");
get_pokedex_text_test!(
    get_pokedex_text_126,
    126,
    "Its body always\nburns with an\norange glow that¶enables it to\nhide perfectly\namong flames"
);
get_pokedex_text_test!(
    get_pokedex_text_127,
    127,
    "If it fails to\ncrush the victim\nin its pincers,¶it will swing it\naround and toss\nit hard"
);
get_pokedex_text_test!(get_pokedex_text_128, 128, "When it targets\nan enemy, it\ncharges furiously¶while whipping its\nbody with its\nlong tails");
get_pokedex_text_test!(get_pokedex_text_129, 129, "In the distant\npast, it was\nsomewhat stronger¶than the horribly\nweak descendants\nthat exist today");
get_pokedex_text_test!(get_pokedex_text_130, 130, "Rarely seen in\nthe wild. Huge\nand vicious, it¶is capable of\ndestroying entire\ncities in a rage");
get_pokedex_text_test!(get_pokedex_text_131, 131, "A #MON that\nhas been over-\nhunted almost to¶extinction. It\ncan ferry people\nacross the water");
get_pokedex_text_test!(get_pokedex_text_132, 132, "Capable of copying\nan enemy's genetic\ncode to instantly¶transform itself\ninto a duplicate\nof the enemy");
get_pokedex_text_test!(get_pokedex_text_133, 133, "Its genetic code\nis irregular.\nIt may mutate if¶it is exposed to\nradiation from\nelement STONEs");
get_pokedex_text_test!(get_pokedex_text_134, 134, "Lives close to\nwater. Its long\ntail is ridged¶with a fin which\nis often mistaken\nfor a mermaid's");
get_pokedex_text_test!(
    get_pokedex_text_135,
    135,
    "It accumulates\nnegative ions in\nthe atmosphere to¶blast out 10000-\nvolt lightning\nbolts"
);
get_pokedex_text_test!(
    get_pokedex_text_136,
    136,
    "When storing\nthermal energy in\nits body, its¶temperature could\nsoar to over 1600\ndegrees"
);
get_pokedex_text_test!(
    get_pokedex_text_137,
    137,
    "A #MON that\nconsists entirely\nof programming¶code. Capable of\nmoving freely in\ncyberspace"
);
get_pokedex_text_test!(
    get_pokedex_text_138,
    138,
    "Although long\nextinct, in rare\ncases, it can be¶genetically\nresurrected from\nfossils"
);
get_pokedex_text_test!(
    get_pokedex_text_139,
    139,
    "A prehistoric\n#MON that died\nout when its¶heavy shell made\nit impossible to\ncatch prey"
);
get_pokedex_text_test!(
    get_pokedex_text_140,
    140,
    "A #MON that\nwas resurrected\nfrom a fossil¶found in what was\nonce the ocean\nfloor eons ago"
);
get_pokedex_text_test!(get_pokedex_text_141, 141, "Its sleek shape is\nperfect for swim-\nming. It slashes¶prey with its\nclaws and drains\nthe body fluids");
get_pokedex_text_test!(get_pokedex_text_142, 142, "A ferocious, pre-\nhistoric #MON\nthat goes for the¶enemy's throat\nwith its serrated\nsaw-like fangs");
get_pokedex_text_test!(get_pokedex_text_143, 143, "Very lazy. Just\neats and sleeps.\nAs its rotund¶bulk builds, it\nbecomes steadily\nmore slothful");
get_pokedex_text_test!(get_pokedex_text_144, 144, "A legendary bird\n#MON that is\nsaid to appear to¶doomed people who\nare lost in icy\nmountains");
get_pokedex_text_test!(get_pokedex_text_145, 145, "A legendary bird\n#MON that is\nsaid to appear¶from clouds while\ndropping enormous\nlightning bolts");
get_pokedex_text_test!(get_pokedex_text_146, 146, "Known as the\nlegendary bird of\nfire. Every flap¶of its wings\ncreates a dazzling\nflash of flames");
get_pokedex_text_test!(get_pokedex_text_147, 147, "Long considered a\nmythical #MON\nuntil recently¶when a small\ncolony was found\nliving underwater");
get_pokedex_text_test!(
    get_pokedex_text_148,
    148,
    "A mystical #MON\nthat exudes a\ngentle aura.¶Has the ability\nto change climate\nconditions"
);
get_pokedex_text_test!(
    get_pokedex_text_149,
    149,
    "An extremely\nrarely seen\nmarine #MON.¶Its intelligence\nis said to match\nthat of humans"
);
get_pokedex_text_test!(get_pokedex_text_150, 150, "It was created by\na scientist after\nyears of horrific¶gene splicing and\nDNA engineering\nexperiments");
get_pokedex_text_test!(get_pokedex_text_151, 151, "So rare that it\nis still said to\nbe a mirage by¶many experts. Only\na few people have\nseen it worldwide");
