import { BaseRequest } from './base';

export type PlayerNamesRequest = BaseRequest<PlayerNamesRequestType, PlayerNamesRequestAttributes>;

enum PlayerNamesRequestType {
    player_names = 'player_names',
}

export interface PlayerNamesRequestAttributes {
    player: Array<string>;
    rival: Array<string>;
}

export const PlayerNamesRequestDefault = (attributes: PlayerNamesRequestAttributes): PlayerNamesRequest => {
    return {
        data: {
            type: PlayerNamesRequestType.player_names,
            attributes,
        },
    };
};
