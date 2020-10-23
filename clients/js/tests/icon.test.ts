import { Pkmnapi } from '../';
import nock from 'nock';

test('getIcon', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/icons/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/gif');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getIcon(0x01);
});
