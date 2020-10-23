import { BaseRequest } from './base';

export type TrainerPartiesRequest = BaseRequest<TrainerPartiesRequestType, TrainerPartiesRequestAttributes>;

enum TrainerPartiesRequestType {
    trainer_parties = 'trainer_parties',
}

export interface TrainerPartiesRequestAttributes {
    parties: Array<TrainerPartiesRequestRequestAttributesParty>;
}

export interface TrainerPartiesRequestRequestAttributesParty {
    party: Array<TrainerPartiesRequestRequestAttributesPartyPokemon>;
}

export interface TrainerPartiesRequestRequestAttributesPartyPokemon {
    level: number;
    pokemon: TrainerPartiesRequestRequestAttributesPartyPokemonAttributes;
}

export interface TrainerPartiesRequestRequestAttributesPartyPokemonAttributes {
    id: string;
}

export const TrainerPartiesRequestDefault = (attributes: TrainerPartiesRequestAttributes): TrainerPartiesRequest => {
    return {
        data: {
            type: TrainerPartiesRequestType.trainer_parties,
            attributes,
        },
    };
};
