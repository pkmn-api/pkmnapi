import { Pkmnapi } from '../';
import nock from 'nock';

test('getPokemonStatsAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/stats')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonStatsAll();
});

test('getPokemonStats', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/pokemon/stats/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPokemonStats(0x01);
});

test('postPokemonStats', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/pokemon/stats/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'pokemon_stats',
                    attributes: {
                        base_hp: 0x02,
                        base_attack: 0x03,
                        base_defence: 0x04,
                        base_speed: 0x05,
                        base_special: 0x06,
                        types: [
                            {
                                id: '7',
                            },
                            {
                                id: '8',
                            },
                        ],
                        catch_rate: 0x09,
                        base_exp_yield: 0x0a,
                        growth_rate: 0x0b,
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postPokemonStats(0x01, {
        base_hp: 0x02,
        base_attack: 0x03,
        base_defence: 0x04,
        base_speed: 0x05,
        base_special: 0x06,
        types: [
            {
                id: '7',
            },
            {
                id: '8',
            },
        ],
        catch_rate: 0x09,
        base_exp_yield: 0x0a,
        growth_rate: 0x0b,
    });
});
