import { Pkmnapi } from '../';
import nock from 'nock';

test('getMapPokemonAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/maps/pokemon')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMapPokemonAll();
});

test('getMapPokemon', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/maps/pokemon/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMapPokemon(0x01);
});

test('postMapPokemon', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/maps/pokemon/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'map_pokemon',
                    attributes: {
                        grass: {
                            encounter_rate: 10,
                            pokemon: [
                                {
                                    level: 10,
                                    pokemon: {
                                        id: '1',
                                    },
                                },
                            ],
                        },
                        water: {
                            encounter_rate: 0,
                            pokemon: [],
                        },
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postMapPokemon(0x01, {
        grass: {
            encounter_rate: 10,
            pokemon: [
                {
                    level: 10,
                    pokemon: {
                        id: '1',
                    },
                },
            ],
        },
        water: {
            encounter_rate: 0,
            pokemon: [],
        },
    });
});
