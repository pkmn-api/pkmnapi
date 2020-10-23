import { Pkmnapi } from '../';
import nock from 'nock';

test('getMoveStatsAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/moves/stats')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMoveStatsAll();
});

test('getMoveStats', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/moves/stats/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMoveStats(0x01);
});

test('postMoveStats', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/moves/stats/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'move_stats',
                    attributes: {
                        effect: 0x02,
                        power: 0x03,
                        type: {
                            id: '4',
                        },
                        accuracy: 0.5,
                        pp: 0x06,
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postMoveStats(0x01, {
        effect: 0x02,
        power: 0x03,
        type: {
            id: '4',
        },
        accuracy: 0.5,
        pp: 0x06,
    });
});
