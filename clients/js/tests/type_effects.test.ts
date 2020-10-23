import { Pkmnapi } from '../';
import nock from 'nock';

test('getTypeEffectAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/types/effects')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTypeEffectAll();
});

test('getTypeEffect', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/types/effects/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTypeEffect(0x01);
});

test('postTypeEffect', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/types/effects/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'type_effects',
                    attributes: {
                        attacking_type: {
                            id: '0',
                        },
                        defending_type: {
                            id: '0',
                        },
                        multiplier: 0.5,
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postTypeEffect(0x01, {
        attacking_type: {
            id: '0',
        },
        defending_type: {
            id: '0',
        },
        multiplier: 0.5,
    });
});
