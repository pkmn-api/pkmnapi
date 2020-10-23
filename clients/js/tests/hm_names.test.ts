import { Pkmnapi } from '../';
import nock from 'nock';

test('getHMNameAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/hms/names')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getHMNameAll();
});

test('getHMName', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/hms/names/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getHMName(0x01);
});
