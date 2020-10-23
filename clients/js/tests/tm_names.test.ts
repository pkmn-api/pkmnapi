import { Pkmnapi } from '../';
import nock from 'nock';

test('getTMNameAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/tms/names')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTMNameAll();
});

test('getTMName', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/tms/names/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTMName(0x01);
});
