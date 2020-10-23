import { Pkmnapi } from '../';
import nock from 'nock';

test('getHMMoveAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/hms/moves')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getHMMoveAll();
});

test('getHMMove', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/hms/moves/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getHMMove(0x01);
});

test('postHMMove', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/hms/moves/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'hm_moves',
                    attributes: {
                        move: {
                            id: '1',
                        },
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postHMMove(0x01, {
        move: {
            id: '1',
        },
    });
});
