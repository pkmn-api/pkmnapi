import axios, { AxiosInstance } from 'axios';

import {
    AccessTokenDeleteRequest,
    AccessTokenDeleteRequestAttributes,
    AccessTokenDeleteRequestDefault,
    AccessTokenRequest,
    AccessTokenRequestAttributes,
    AccessTokenRequestDefault,
    HMMoveRequest,
    HMMoveRequestAttributes,
    HMMoveRequestDefault,
    ItemNameRequest,
    ItemNameRequestAttributes,
    ItemNameRequestDefault,
    MapPokemonRequest,
    MapPokemonRequestAttributes,
    MapPokemonRequestDefault,
    MartItemsRequest,
    MartItemsRequestAttributes,
    MartItemsRequestDefault,
    MoveNameRequest,
    MoveNameRequestAttributes,
    MoveNameRequestDefault,
    MoveStatsRequest,
    MoveStatsRequestAttributes,
    MoveStatsRequestDefault,
    PlayerNamesRequest,
    PlayerNamesRequestAttributes,
    PlayerNamesRequestDefault,
    PokedexEntryRequest,
    PokedexEntryRequestAttributes,
    PokedexEntryRequestDefault,
    PokedexTextRequest,
    PokedexTextRequestAttributes,
    PokedexTextRequestDefault,
    PokemonEvolutionsRequest,
    PokemonEvolutionsRequestAttributes,
    PokemonEvolutionsRequestDefault,
    PokemonIconRequest,
    PokemonIconRequestAttributes,
    PokemonIconRequestDefault,
    PokemonLearnsetRequest,
    PokemonLearnsetRequestAttributes,
    PokemonLearnsetRequestDefault,
    PokemonMachinesRequest,
    PokemonMachinesRequestAttributes,
    PokemonMachinesRequestDefault,
    PokemonMovesetRequest,
    PokemonMovesetRequestAttributes,
    PokemonMovesetRequestDefault,
    PokemonNameRequest,
    PokemonNameRequestAttributes,
    PokemonNameRequestDefault,
    PokemonStatsRequest,
    PokemonStatsRequestAttributes,
    PokemonStatsRequestDefault,
    TMMoveRequest,
    TMMoveRequestAttributes,
    TMMoveRequestDefault,
    TradeRequest,
    TradeRequestAttributes,
    TradeRequestDefault,
    TrainerNameRequest,
    TrainerNameRequestAttributes,
    TrainerNameRequestDefault,
    TrainerPartiesRequest,
    TrainerPartiesRequestAttributes,
    TrainerPartiesRequestDefault,
    TrainerRewardRequest,
    TrainerRewardRequestAttributes,
    TrainerRewardRequestDefault,
    TypeEffectRequest,
    TypeEffectRequestAttributes,
    TypeEffectRequestDefault,
    TypeNameRequest,
    TypeNameRequestAttributes,
    TypeNameRequestDefault,
} from './requests/index';

import {
    HMMoveResponse,
    HMMoveResponseAll,
    HMNameResponse,
    HMNameResponseAll,
    ItemNameResponse,
    ItemNameResponseAll,
    MapPokemonResponse,
    MapPokemonResponseAll,
    MartItemsResponse,
    MartItemsResponseAll,
    MoveNameResponse,
    MoveNameResponseAll,
    MoveStatsResponse,
    MoveStatsResponseAll,
    PlayerNamesResponse,
    PokedexEntryResponse,
    PokedexEntryResponseAll,
    PokedexTextResponse,
    PokedexTextResponseAll,
    PokemonEvolutionsResponse,
    PokemonEvolutionsResponseAll,
    PokemonIconResponse,
    PokemonIconResponseAll,
    PokemonLearnsetResponse,
    PokemonLearnsetResponseAll,
    PokemonMachinesResponse,
    PokemonMachinesResponseAll,
    PokemonMovesetResponse,
    PokemonMovesetResponseAll,
    PokemonNameResponse,
    PokemonNameResponseAll,
    PokemonStatsResponse,
    PokemonStatsResponseAll,
    TMMoveResponse,
    TMMoveResponseAll,
    TMNameResponse,
    TMNameResponseAll,
    TradeResponse,
    TradeResponseAll,
    TrainerNameResponse,
    TrainerNameResponseAll,
    TrainerPartiesResponse,
    TrainerPartiesResponseAll,
    TrainerRewardResponse,
    TrainerRewardResponseAll,
    TypeEffectResponse,
    TypeEffectResponseAll,
    TypeNameResponse,
    TypeNameResponseAll,
} from './responses/index';

export interface PkmnapiConfig {
    api_domain: string;
    api_version: number;
    access_token: string;
}

const PkmnapiConfigDefault: PkmnapiConfig = {
    api_domain: 'https://api.pkmnapi.com',
    api_version: 1,
    access_token: null,
};

class Pkmnapi {
    config: PkmnapiConfig;
    client: AxiosInstance;

    constructor(config?: PkmnapiConfig) {
        this.config = {
            ...PkmnapiConfigDefault,
            ...config,
        };

        const baseURL = [this.config.api_domain, `v${this.config.api_version}`].join('/');

        this.client = axios.create({
            baseURL: baseURL,
            headers: {
                Accept: 'application/json',
            },
        });
    }

    _endpoint(...endpoint: string[]): string {
        return endpoint.join('/');
    }

    _handleErr(err: any): Promise<any> {
        let data = err.toString();

        try {
            data = err.response.data;
        } catch (_) {}

        return Promise.reject(data);
    }

    _getRequest<T>(endpoint: string): Promise<T> {
        return this.client
            .get(endpoint)
            .then((res) => {
                const data: T = res.data;

                return Promise.resolve(data);
            })
            .catch((err) => {
                return this._handleErr(err);
            });
    }

    _getRequestRaw(endpoint: string, content_type: string): Promise<Uint8Array> {
        return this.client
            .get(endpoint, {
                responseType: 'arraybuffer',
                headers: {
                    Accept: content_type,
                },
            })
            .then((res) => {
                const data = new Uint8Array(res.data);

                return Promise.resolve(data);
            })
            .catch((err) => {
                return this._handleErr(err);
            });
    }

    _postRequest<T, U>(endpoint: string, data: T): Promise<U> {
        return this.client
            .post(endpoint, data, {
                headers: {
                    'Content-Type': 'application/json',
                },
            })
            .then((res) => {
                const data: U = res.data;

                return Promise.resolve(data);
            })
            .catch((err) => {
                return this._handleErr(err);
            });
    }

    _postRequestRaw(endpoint: string, content_type: string, data: Uint8Array): Promise<unknown> {
        return this.client
            .post(endpoint, data, {
                headers: {
                    'Content-Type': content_type,
                },
            })
            .then((res) => {
                const data: unknown = res.data;

                return Promise.resolve(data);
            })
            .catch((err) => {
                return this._handleErr(err);
            });
    }

    _deleteRequest<T, U>(endpoint: string, data: T): Promise<U> {
        return this.client
            .delete(endpoint, {
                headers: {
                    'Content-Type': 'application/json',
                },
                data: {
                    source: data,
                },
            })
            .then((res) => {
                const data: U = res.data;

                return Promise.resolve(data);
            })
            .catch((err) => {
                return this._handleErr(err);
            });
    }

    setAccessToken(access_token: string): Pkmnapi {
        this.client.defaults.headers.common = {
            ...this.client.defaults.headers.common,
            Authorization: `Bearer ${access_token}`,
        };

        return this;
    }

    // access_tokens
    postAccessToken(attributes: AccessTokenRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('access_tokens');
        const data = AccessTokenRequestDefault(attributes);

        return this._postRequest<AccessTokenRequest, unknown>(endpoint, data);
    }

    postAccessTokenDelete(attributes: AccessTokenRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('access_tokens', 'delete');
        const data = AccessTokenRequestDefault(attributes);

        return this._postRequest<AccessTokenRequest, unknown>(endpoint, data);
    }

    deleteAccessToken(attributes: AccessTokenDeleteRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('access_tokens');
        const data = AccessTokenDeleteRequestDefault(attributes);

        return this._deleteRequest<AccessTokenDeleteRequest, unknown>(endpoint, data);
    }

    // hm_moves
    getHMMoveAll(): Promise<HMMoveResponseAll> {
        const endpoint = this._endpoint('hms', 'moves');

        return this._getRequest<HMMoveResponseAll>(endpoint);
    }

    getHMMove(hm_id: number): Promise<HMMoveResponse> {
        const endpoint = this._endpoint('hms', 'moves', hm_id.toString());

        return this._getRequest<HMMoveResponse>(endpoint);
    }

    postHMMove(hm_id: number, attributes: HMMoveRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('hms', 'moves', hm_id.toString());
        const data = HMMoveRequestDefault(attributes);

        return this._postRequest<HMMoveRequest, unknown>(endpoint, data);
    }

    // hm_names
    getHMNameAll(): Promise<HMNameResponseAll> {
        const endpoint = this._endpoint('hms', 'names');

        return this._getRequest<HMNameResponseAll>(endpoint);
    }

    getHMName(hm_id: number): Promise<HMNameResponse> {
        const endpoint = this._endpoint('hms', 'names', hm_id.toString());

        return this._getRequest<HMNameResponse>(endpoint);
    }

    // icons
    getIcon(icon_id: number): Promise<Uint8Array> {
        const endpoint = this._endpoint('icons', icon_id.toString());

        return this._getRequestRaw(endpoint, 'image/gif');
    }

    // imgs
    getImgPokemonLogo(content_type: string): Promise<Uint8Array> {
        const endpoint = this._endpoint('imgs', 'pokemon_logo');

        return this._getRequestRaw(endpoint, content_type);
    }

    postImgPokemonLogo(content_type: string, data: Uint8Array): Promise<unknown> {
        const endpoint = this._endpoint('imgs', 'pokemon_logo');

        return this._postRequestRaw(endpoint, content_type, data);
    }

    getImgTownMap(content_type: string): Promise<Uint8Array> {
        const endpoint = this._endpoint('imgs', 'town_map');

        return this._getRequestRaw(endpoint, content_type);
    }

    // item_names
    getItemNameAll(): Promise<ItemNameResponseAll> {
        const endpoint = this._endpoint('items', 'names');

        return this._getRequest<ItemNameResponseAll>(endpoint);
    }

    getItemName(item_id: number): Promise<ItemNameResponse> {
        const endpoint = this._endpoint('items', 'names', item_id.toString());

        return this._getRequest<ItemNameResponse>(endpoint);
    }

    postItemName(item_id: number, attributes: ItemNameRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('items', 'names', item_id.toString());
        const data = ItemNameRequestDefault(attributes);

        return this._postRequest<ItemNameRequest, unknown>(endpoint, data);
    }

    // map_pics
    getMapPic(map_id: number, content_type: string): Promise<Uint8Array> {
        const endpoint = this._endpoint('maps', 'pics', map_id.toString());

        return this._getRequestRaw(endpoint, content_type);
    }

    // map_pokemon
    getMapPokemonAll(): Promise<MapPokemonResponseAll> {
        const endpoint = this._endpoint('maps', 'pokemon');

        return this._getRequest<MapPokemonResponseAll>(endpoint);
    }

    getMapPokemon(map_id: number): Promise<MapPokemonResponse> {
        const endpoint = this._endpoint('maps', 'pokemon', map_id.toString());

        return this._getRequest<MapPokemonResponse>(endpoint);
    }

    postMapPokemon(map_id: number, attributes: MapPokemonRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('maps', 'pokemon', map_id.toString());
        const data = MapPokemonRequestDefault(attributes);

        return this._postRequest<MapPokemonRequest, unknown>(endpoint, data);
    }

    // mart_items
    getMartItemsAll(): Promise<MartItemsResponseAll> {
        const endpoint = this._endpoint('marts', 'items');

        return this._getRequest<MartItemsResponseAll>(endpoint);
    }

    getMartItems(move_id: number): Promise<MartItemsResponse> {
        const endpoint = this._endpoint('marts', 'items', move_id.toString());

        return this._getRequest<MartItemsResponse>(endpoint);
    }

    postMartItems(move_id: number, attributes: MartItemsRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('marts', 'items', move_id.toString());
        const data = MartItemsRequestDefault(attributes);

        return this._postRequest<MartItemsRequest, unknown>(endpoint, data);
    }

    // move_names
    getMoveNameAll(): Promise<MoveNameResponseAll> {
        const endpoint = this._endpoint('moves', 'names');

        return this._getRequest<MoveNameResponseAll>(endpoint);
    }

    getMoveName(move_id: number): Promise<MoveNameResponse> {
        const endpoint = this._endpoint('moves', 'names', move_id.toString());

        return this._getRequest<MoveNameResponse>(endpoint);
    }

    postMoveName(move_id: number, attributes: MoveNameRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('moves', 'names', move_id.toString());
        const data = MoveNameRequestDefault(attributes);

        return this._postRequest<MoveNameRequest, unknown>(endpoint, data);
    }

    // move_stats
    getMoveStatsAll(): Promise<MoveStatsResponseAll> {
        const endpoint = this._endpoint('moves', 'stats');

        return this._getRequest<MoveStatsResponseAll>(endpoint);
    }

    getMoveStats(move_id: number): Promise<MoveStatsResponse> {
        const endpoint = this._endpoint('moves', 'stats', move_id.toString());

        return this._getRequest<MoveStatsResponse>(endpoint);
    }

    postMoveStats(move_id: number, attributes: MoveStatsRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('moves', 'stats', move_id.toString());
        const data = MoveStatsRequestDefault(attributes);

        return this._postRequest<MoveStatsRequest, unknown>(endpoint, data);
    }

    // player_names
    getPlayerNames(): Promise<PlayerNamesResponse> {
        const endpoint = this._endpoint('player_names');

        return this._getRequest<PlayerNamesResponse>(endpoint);
    }

    postPlayerNames(attributes: PlayerNamesRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('player_names');
        const data = PlayerNamesRequestDefault(attributes);

        return this._postRequest<PlayerNamesRequest, unknown>(endpoint, data);
    }

    // pokedex_entries
    getPokedexEntryAll(): Promise<PokedexEntryResponseAll> {
        const endpoint = this._endpoint('pokedex', 'entries');

        return this._getRequest<PokedexEntryResponseAll>(endpoint);
    }

    getPokedexEntry(pokedex_id: number): Promise<PokedexEntryResponse> {
        const endpoint = this._endpoint('pokedex', 'entries', pokedex_id.toString());

        return this._getRequest<PokedexEntryResponse>(endpoint);
    }

    postPokedexEntry(pokedex_id: number, attributes: PokedexEntryRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokedex', 'entries', pokedex_id.toString());
        const data = PokedexEntryRequestDefault(attributes);

        return this._postRequest<PokedexEntryRequest, unknown>(endpoint, data);
    }

    // pokedex_texts
    getPokedexTextAll(): Promise<PokedexTextResponseAll> {
        const endpoint = this._endpoint('pokedex', 'texts');

        return this._getRequest<PokedexTextResponseAll>(endpoint);
    }

    getPokedexText(pokedex_id: number): Promise<PokedexTextResponse> {
        const endpoint = this._endpoint('pokedex', 'texts', pokedex_id.toString());

        return this._getRequest<PokedexTextResponse>(endpoint);
    }

    postPokedexText(pokedex_id: number, attributes: PokedexTextRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokedex', 'texts', pokedex_id.toString());
        const data = PokedexTextRequestDefault(attributes);

        return this._postRequest<PokedexTextRequest, unknown>(endpoint, data);
    }

    // TODO

    // pokemon_evolutions
    getPokemonEvolutionsAll(): Promise<PokemonEvolutionsResponseAll> {
        const endpoint = this._endpoint('pokemon', 'evolutions');

        return this._getRequest<PokemonEvolutionsResponseAll>(endpoint);
    }

    getPokemonEvolutions(pokedex_id: number): Promise<PokemonEvolutionsResponse> {
        const endpoint = this._endpoint('pokemon', 'evolutions', pokedex_id.toString());

        return this._getRequest<PokemonEvolutionsResponse>(endpoint);
    }

    postPokemonEvolutions(pokedex_id: number, attributes: PokemonEvolutionsRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'evolutions', pokedex_id.toString());
        const data = PokemonEvolutionsRequestDefault(attributes);

        return this._postRequest<PokemonEvolutionsRequest, unknown>(endpoint, data);
    }

    // pokemon_icons
    getPokemonIconAll(): Promise<PokemonIconResponseAll> {
        const endpoint = this._endpoint('pokemon', 'icons');

        return this._getRequest<PokemonIconResponseAll>(endpoint);
    }

    getPokemonIcon(pokedex_id: number): Promise<PokemonIconResponse> {
        const endpoint = this._endpoint('pokemon', 'icons', pokedex_id.toString());

        return this._getRequest<PokemonIconResponse>(endpoint);
    }

    postPokemonIcon(pokedex_id: number, attributes: PokemonIconRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'icons', pokedex_id.toString());
        const data = PokemonIconRequestDefault(attributes);

        return this._postRequest<PokemonIconRequest, unknown>(endpoint, data);
    }

    // pokemon_learnsets
    getPokemonLearnsetAll(): Promise<PokemonLearnsetResponseAll> {
        const endpoint = this._endpoint('pokemon', 'learnsets');

        return this._getRequest<PokemonLearnsetResponseAll>(endpoint);
    }

    getPokemonLearnset(pokedex_id: number): Promise<PokemonLearnsetResponse> {
        const endpoint = this._endpoint('pokemon', 'learnsets', pokedex_id.toString());

        return this._getRequest<PokemonLearnsetResponse>(endpoint);
    }

    postPokemonLearnset(pokedex_id: number, attributes: PokemonLearnsetRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'learnsets', pokedex_id.toString());
        const data = PokemonLearnsetRequestDefault(attributes);

        return this._postRequest<PokemonLearnsetRequest, unknown>(endpoint, data);
    }

    // pokemon_machines
    getPokemonMachinesAll(): Promise<PokemonMachinesResponseAll> {
        const endpoint = this._endpoint('pokemon', 'machines');

        return this._getRequest<PokemonMachinesResponseAll>(endpoint);
    }

    getPokemonMachines(pokedex_id: number): Promise<PokemonMachinesResponse> {
        const endpoint = this._endpoint('pokemon', 'machines', pokedex_id.toString());

        return this._getRequest<PokemonMachinesResponse>(endpoint);
    }

    postPokemonMachines(pokedex_id: number, attributes: PokemonMachinesRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'machines', pokedex_id.toString());
        const data = PokemonMachinesRequestDefault(attributes);

        return this._postRequest<PokemonMachinesRequest, unknown>(endpoint, data);
    }

    // pokemon_movesets
    getPokemonMovesetAll(): Promise<PokemonMovesetResponseAll> {
        const endpoint = this._endpoint('pokemon', 'movesets');

        return this._getRequest<PokemonMovesetResponseAll>(endpoint);
    }

    getPokemonMoveset(pokedex_id: number): Promise<PokemonMovesetResponse> {
        const endpoint = this._endpoint('pokemon', 'movesets', pokedex_id.toString());

        return this._getRequest<PokemonMovesetResponse>(endpoint);
    }

    postPokemonMoveset(pokedex_id: number, attributes: PokemonMovesetRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'movesets', pokedex_id.toString());
        const data = PokemonMovesetRequestDefault(attributes);

        return this._postRequest<PokemonMovesetRequest, unknown>(endpoint, data);
    }

    // pokemon_names
    getPokemonNameAll(): Promise<PokemonNameResponseAll> {
        const endpoint = this._endpoint('pokemon', 'names');

        return this._getRequest<PokemonNameResponseAll>(endpoint);
    }

    getPokemonName(pokedex_id: number): Promise<PokemonNameResponse> {
        const endpoint = this._endpoint('pokemon', 'names', pokedex_id.toString());

        return this._getRequest<PokemonNameResponse>(endpoint);
    }

    postPokemonName(pokedex_id: number, attributes: PokemonNameRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'names', pokedex_id.toString());
        const data = PokemonNameRequestDefault(attributes);

        return this._postRequest<PokemonNameRequest, unknown>(endpoint, data);
    }

    // TODO

    // pokemon_stats
    getPokemonStatsAll(): Promise<PokemonStatsResponseAll> {
        const endpoint = this._endpoint('pokemon', 'stats');

        return this._getRequest<PokemonStatsResponseAll>(endpoint);
    }

    getPokemonStats(pokedex_id: number): Promise<PokemonStatsResponse> {
        const endpoint = this._endpoint('pokemon', 'stats', pokedex_id.toString());

        return this._getRequest<PokemonStatsResponse>(endpoint);
    }

    postPokemonStats(pokedex_id: number, attributes: PokemonStatsRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('pokemon', 'stats', pokedex_id.toString());
        const data = PokemonStatsRequestDefault(attributes);

        return this._postRequest<PokemonStatsRequest, unknown>(endpoint, data);
    }

    // TODO

    // tm_moves
    getTMMoveAll(): Promise<TMMoveResponseAll> {
        const endpoint = this._endpoint('tms', 'moves');

        return this._getRequest<TMMoveResponseAll>(endpoint);
    }

    getTMMove(tm_id: number): Promise<TMMoveResponse> {
        const endpoint = this._endpoint('tms', 'moves', tm_id.toString());

        return this._getRequest<TMMoveResponse>(endpoint);
    }

    postTMMove(tm_id: number, attributes: TMMoveRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('tms', 'moves', tm_id.toString());
        const data = TMMoveRequestDefault(attributes);

        return this._postRequest<TMMoveRequest, unknown>(endpoint, data);
    }

    // tm_names
    getTMNameAll(): Promise<TMNameResponseAll> {
        const endpoint = this._endpoint('tms', 'names');

        return this._getRequest<TMNameResponseAll>(endpoint);
    }

    getTMName(tm_id: number): Promise<TMNameResponse> {
        const endpoint = this._endpoint('tms', 'names', tm_id.toString());

        return this._getRequest<TMNameResponse>(endpoint);
    }

    // trades
    getTradeAll(): Promise<TradeResponseAll> {
        const endpoint = this._endpoint('trades');

        return this._getRequest<TradeResponseAll>(endpoint);
    }

    getTrade(trade_id: number): Promise<TradeResponse> {
        const endpoint = this._endpoint('trades', trade_id.toString());

        return this._getRequest<TradeResponse>(endpoint);
    }

    postTrade(trade_id: number, attributes: TradeRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('trades', trade_id.toString());
        const data = TradeRequestDefault(attributes);

        return this._postRequest<TradeRequest, unknown>(endpoint, data);
    }

    // trainer_names
    getTrainerNameAll(): Promise<TrainerNameResponseAll> {
        const endpoint = this._endpoint('trainers', 'names');

        return this._getRequest<TrainerNameResponseAll>(endpoint);
    }

    getTrainerName(trainer_id: number): Promise<TrainerNameResponse> {
        const endpoint = this._endpoint('trainers', 'names', trainer_id.toString());

        return this._getRequest<TrainerNameResponse>(endpoint);
    }

    postTrainerName(trainer_id: number, attributes: TrainerNameRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('trainers', 'names', trainer_id.toString());
        const data = TrainerNameRequestDefault(attributes);

        return this._postRequest<TrainerNameRequest, unknown>(endpoint, data);
    }

    // trainer_parties
    getTrainerPartiesAll(): Promise<TrainerPartiesResponseAll> {
        const endpoint = this._endpoint('trainers', 'parties');

        return this._getRequest<TrainerPartiesResponseAll>(endpoint);
    }

    getTrainerParties(trainer_id: number): Promise<TrainerPartiesResponse> {
        const endpoint = this._endpoint('trainers', 'parties', trainer_id.toString());

        return this._getRequest<TrainerPartiesResponse>(endpoint);
    }

    postTrainerParties(trainer_id: number, attributes: TrainerPartiesRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('trainers', 'parties', trainer_id.toString());
        const data = TrainerPartiesRequestDefault(attributes);

        return this._postRequest<TrainerPartiesRequest, unknown>(endpoint, data);
    }

    // TODO

    // trainer_rewards
    getTrainerRewardAll(): Promise<TrainerRewardResponseAll> {
        const endpoint = this._endpoint('trainers', 'rewards');

        return this._getRequest<TrainerRewardResponseAll>(endpoint);
    }

    getTrainerReward(trainer_id: number): Promise<TrainerRewardResponse> {
        const endpoint = this._endpoint('trainers', 'rewards', trainer_id.toString());

        return this._getRequest<TrainerRewardResponse>(endpoint);
    }

    postTrainerReward(trainer_id: number, attributes: TrainerRewardRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('trainers', 'rewards', trainer_id.toString());
        const data = TrainerRewardRequestDefault(attributes);

        return this._postRequest<TrainerRewardRequest, unknown>(endpoint, data);
    }

    // type_effects
    getTypeEffectAll(): Promise<TypeEffectResponseAll> {
        const endpoint = this._endpoint('types', 'effects');

        return this._getRequest<TypeEffectResponseAll>(endpoint);
    }

    getTypeEffect(type_effect_id: number): Promise<TypeEffectResponse> {
        const endpoint = this._endpoint('types', 'effects', type_effect_id.toString());

        return this._getRequest<TypeEffectResponse>(endpoint);
    }

    postTypeEffect(type_effect_id: number, attributes: TypeEffectRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('types', 'effects', type_effect_id.toString());
        const data = TypeEffectRequestDefault(attributes);

        return this._postRequest<TypeEffectRequest, unknown>(endpoint, data);
    }

    // type_names
    getTypeNameAll(): Promise<TypeNameResponseAll> {
        const endpoint = this._endpoint('types', 'names');

        return this._getRequest<TypeNameResponseAll>(endpoint);
    }

    getTypeName(type_id: number): Promise<TypeNameResponse> {
        const endpoint = this._endpoint('types', 'names', type_id.toString());

        return this._getRequest<TypeNameResponse>(endpoint);
    }

    postTypeName(type_id: number, attributes: TypeNameRequestAttributes): Promise<unknown> {
        const endpoint = this._endpoint('types', 'names', type_id.toString());
        const data = TypeNameRequestDefault(attributes);

        return this._postRequest<TypeNameRequest, unknown>(endpoint, data);
    }
}

export { Pkmnapi };
