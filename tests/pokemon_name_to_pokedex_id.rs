use pkmnapi::db::string::*;
use pkmnapi::db::types::*;

mod common;

macro_rules! pokemon_name_to_pokedex_id_test {
    ($test_name: ident, $pokemon_name: expr, $pokedex_id: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.pokemon_name_to_pokedex_id(PokemonName {
                name: ROMString::from($pokemon_name),
            }) {
                Some(pokedex_id) => assert_eq!(
                    pokedex_id, $pokedex_id,
                    "Searched for Pokémon name: {}",
                    $pokemon_name
                ),
                None => panic!(format!("Could not find Pokémon name: {}", $pokemon_name)),
            };
        }
    };
}

#[rustfmt::skip::macros(pokemon_name_to_pokedex_id_test)]

pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_bulbasaur, "BULBASAUR", 1);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_ivysaur, "IVYSAUR", 2);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_venusaur, "VENUSAUR", 3);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_charmander, "CHARMANDER", 4);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_charmeleon, "CHARMELEON", 5);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_charizard, "CHARIZARD", 6);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_squirtle, "SQUIRTLE", 7);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_wartortle, "WARTORTLE", 8);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_blastoise, "BLASTOISE", 9);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_caterpie, "CATERPIE", 10);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_metapod, "METAPOD", 11);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_butterfree, "BUTTERFREE", 12);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_weedle, "WEEDLE", 13);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_kakuna, "KAKUNA", 14);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_beedrill, "BEEDRILL", 15);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_pidgey, "PIDGEY", 16);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_pidgeotto, "PIDGEOTTO", 17);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_pidgeot, "PIDGEOT", 18);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_rattata, "RATTATA", 19);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_raticate, "RATICATE", 20);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_spearow, "SPEAROW", 21);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_fearow, "FEAROW", 22);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_ekans, "EKANS", 23);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_arbok, "ARBOK", 24);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_pikachu, "PIKACHU", 25);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_raichu, "RAICHU", 26);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_sandshrew, "SANDSHREW", 27);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_sandslash, "SANDSLASH", 28);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_nidoranf, "NIDORAN♀", 29);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_nidorina, "NIDORINA", 30);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_nidoqueen, "NIDOQUEEN", 31);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_nidoranm, "NIDORAN♂", 32);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_nidorino, "NIDORINO", 33);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_nidoking, "NIDOKING", 34);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_clefairy, "CLEFAIRY", 35);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_clefable, "CLEFABLE", 36);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_vulpix, "VULPIX", 37);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_ninetales, "NINETALES", 38);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_jigglypuff, "JIGGLYPUFF", 39);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_wigglytuff, "WIGGLYTUFF", 40);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_zubat, "ZUBAT", 41);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_golbat, "GOLBAT", 42);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_oddish, "ODDISH", 43);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_gloom, "GLOOM", 44);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_vileplume, "VILEPLUME", 45);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_paras, "PARAS", 46);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_parasect, "PARASECT", 47);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_venonat, "VENONAT", 48);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_venomoth, "VENOMOTH", 49);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_diglett, "DIGLETT", 50);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_dugtrio, "DUGTRIO", 51);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_meowth, "MEOWTH", 52);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_persian, "PERSIAN", 53);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_psyduck, "PSYDUCK", 54);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_golduck, "GOLDUCK", 55);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_mankey, "MANKEY", 56);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_primeape, "PRIMEAPE", 57);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_growlithe, "GROWLITHE", 58);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_arcanine, "ARCANINE", 59);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_poliwag, "POLIWAG", 60);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_poliwhirl, "POLIWHIRL", 61);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_poliwrath, "POLIWRATH", 62);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_abra, "ABRA", 63);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_kadabra, "KADABRA", 64);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_alakazam, "ALAKAZAM", 65);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_machop, "MACHOP", 66);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_machoke, "MACHOKE", 67);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_machamp, "MACHAMP", 68);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_bellsprout, "BELLSPROUT", 69);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_weepinbell, "WEEPINBELL", 70);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_victreebel, "VICTREEBEL", 71);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_tentacool, "TENTACOOL", 72);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_tentacruel, "TENTACRUEL", 73);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_geodude, "GEODUDE", 74);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_graveler, "GRAVELER", 75);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_golem, "GOLEM", 76);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_ponyta, "PONYTA", 77);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_rapidash, "RAPIDASH", 78);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_slowpoke, "SLOWPOKE", 79);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_slowbro, "SLOWBRO", 80);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_magnemite, "MAGNEMITE", 81);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_magneton, "MAGNETON", 82);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_farfetchd, "FARFETCH'D", 83);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_doduo, "DODUO", 84);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_dodrio, "DODRIO", 85);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_seel, "SEEL", 86);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_dewgong, "DEWGONG", 87);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_grimer, "GRIMER", 88);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_muk, "MUK", 89);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_shellder, "SHELLDER", 90);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_cloyster, "CLOYSTER", 91);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_gastly, "GASTLY", 92);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_haunter, "HAUNTER", 93);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_gengar, "GENGAR", 94);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_onix, "ONIX", 95);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_drowzee, "DROWZEE", 96);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_hypno, "HYPNO", 97);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_krabby, "KRABBY", 98);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_kingler, "KINGLER", 99);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_voltorb, "VOLTORB", 100);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_electrode, "ELECTRODE", 101);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_exeggcute, "EXEGGCUTE", 102);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_exeggutor, "EXEGGUTOR", 103);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_cubone, "CUBONE", 104);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_marowak, "MAROWAK", 105);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_hitmonlee, "HITMONLEE", 106);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_hitmonchan, "HITMONCHAN", 107);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_lickitung, "LICKITUNG", 108);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_koffing, "KOFFING", 109);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_weezing, "WEEZING", 110);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_rhyhorn, "RHYHORN", 111);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_rhydon, "RHYDON", 112);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_chansey, "CHANSEY", 113);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_tangela, "TANGELA", 114);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_kangaskhan, "KANGASKHAN", 115);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_horsea, "HORSEA", 116);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_seadra, "SEADRA", 117);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_goldeen, "GOLDEEN", 118);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_seaking, "SEAKING", 119);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_staryu, "STARYU", 120);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_starmie, "STARMIE", 121);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_mrmime, "MR.MIME", 122);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_scyther, "SCYTHER", 123);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_jynx, "JYNX", 124);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_electabuzz, "ELECTABUZZ", 125);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_magmar, "MAGMAR", 126);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_pinsir, "PINSIR", 127);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_tauros, "TAUROS", 128);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_magikarp, "MAGIKARP", 129);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_gyarados, "GYARADOS", 130);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_lapras, "LAPRAS", 131);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_ditto, "DITTO", 132);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_eevee, "EEVEE", 133);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_vaporeon, "VAPOREON", 134);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_jolteon, "JOLTEON", 135);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_flareon, "FLAREON", 136);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_porygon, "PORYGON", 137);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_omanyte, "OMANYTE", 138);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_omastar, "OMASTAR", 139);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_kabuto, "KABUTO", 140);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_kabutops, "KABUTOPS", 141);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_aerodactyl, "AERODACTYL", 142);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_snorlax, "SNORLAX", 143);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_articuno, "ARTICUNO", 144);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_zapdos, "ZAPDOS", 145);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_moltres, "MOLTRES", 146);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_dratini, "DRATINI", 147);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_dragonair, "DRAGONAIR", 148);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_dragonite, "DRAGONITE", 149);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_mewtwo, "MEWTWO", 150);
pokemon_name_to_pokedex_id_test!(pokemon_name_to_pokedex_id_mew, "MEW", 151);
