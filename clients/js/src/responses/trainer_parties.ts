import { BaseResponse, BaseResponseData, BaseResponseAll } from './base';
import { PokemonNameResponseData } from './pokemon_names';

export type TrainerPartiesResponse = BaseResponse<TrainerPartiesResponseAttributes>;
export type TrainerPartiesResponseData = BaseResponseData<TrainerPartiesResponseAttributes>;
export type TrainerPartiesResponseAll = BaseResponseAll<TrainerPartiesResponseData>;

export interface TrainerPartiesResponseAttributes {
    parties: Array<TrainerPartiesResponseAttributesParty>;
}

export interface TrainerPartiesResponseAttributesParty {
    party: Array<TrainerPartiesResponseAttributesPartyPokemon>;
}

export interface TrainerPartiesResponseAttributesPartyPokemon {
    level: number;
    pokemon: PokemonNameResponseData;
}
