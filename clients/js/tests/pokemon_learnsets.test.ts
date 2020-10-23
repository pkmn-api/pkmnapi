import { Pkmnapi } from '../';
import nock from 'nock';

test('getPokemonLearnsetAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/learnsets')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonLearnsetAll();
});

test('getPokemonLearnset', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/learnsets/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonLearnset(0x01);
});

test('postPokemonLearnset', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/pokemon/learnsets/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'pokemon_learnsets',
                    attributes: {
                        learnset: [
                            {
                                level: 5,
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

    pkmnapi.postPokemonLearnset(0x01, {
        learnset: [
            {
                level: 5,
                move: {
                    id: '1',
                },
            },
        ],
    });
});
