import { Pkmnapi } from '../';
import nock from 'nock';

test('getMapPicJPEG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/maps/pics/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/jpeg');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMapPic(0x01, 'image/jpeg');
});

test('getMapPicPNG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/maps/pics/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/png');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMapPic(0x01, 'image/png');
});
