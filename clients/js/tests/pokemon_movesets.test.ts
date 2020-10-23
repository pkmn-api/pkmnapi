import { Pkmnapi } from '../';
import nock from 'nock';

test('getPokemonMovesetAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/movesets')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonMovesetAll();
});

test('getPokemonMoveset', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/movesets/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonMoveset(0x01);
});

test('postPokemonMoveset', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/pokemon/movesets/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'pokemon_movesets',
                    attributes: {
                        moveset: [
                            {
                                move: {
                                    id: '1',
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

    pkmnapi.postPokemonMoveset(0x01, {
        moveset: [
            {
                move: {
                    id: '1',
                },
            },
        ],
    });
});
