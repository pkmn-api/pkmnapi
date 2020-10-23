import { Pkmnapi } from '../';
import nock from 'nock';

test('getPlayerNames', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/player_names')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getPlayerNames();
});

test('postPlayerNames', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/player_names')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'player_names',
                    attributes: {
                        player: ['FOO', 'BAR', 'BAZ'],
                        rival: ['FOO', 'BAR', 'BAZ'],
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postPlayerNames({
        player: ['FOO', 'BAR', 'BAZ'],
        rival: ['FOO', 'BAR', 'BAZ'],
    });
});
