import { BaseRequest } from './base';

export type PokemonMachinesRequest = BaseRequest<PokemonMachinesRequestType, PokemonMachinesRequestAttributes>;

enum PokemonMachinesRequestType {
    pokemon_machines = 'pokemon_machines',
}

export interface PokemonMachinesRequestAttributes {
    machines: Array<PokemonMachinesRequestAttributesMachine>;
}

export interface PokemonMachinesRequestAttributesMachine {
    id: string;
    type: PokemonMachinesRequestAttributesMachineType | string;
}

enum PokemonMachinesRequestAttributesMachineType {
    tm_moves = 'tm_moves',
    hm_moves = 'hm_moves',
}

export const PokemonMachinesRequestDefault = (attributes: PokemonMachinesRequestAttributes): PokemonMachinesRequest => {
    return {
        data: {
            type: PokemonMachinesRequestType.pokemon_machines,
            attributes,
        },
    };
};
