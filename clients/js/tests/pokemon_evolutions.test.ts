import { Pkmnapi } from '../';
import nock from 'nock';

test('getPokemonEvolutionsAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/evolutions')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonEvolutionsAll();
});

test('getPokemonEvolutions', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/evolutions/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonEvolutions(0x01);
});

test('postPokemonEvolutions', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/pokemon/evolutions/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'pokemon_evolutions',
                    attributes: {
                        evolutions: [
                            {
                                evolution_type: 'level',
                                level: 5,
                                pokemon: {
                                    id: '2',
                                },
                            },
                        ],
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postPokemonEvolutions(0x01, {
        evolutions: [
            {
                evolution_type: 'level',
                level: 5,
                pokemon: {
                    id: '2',
                },
            },
        ],
    });
});
