import { Pkmnapi } from '../';
import nock from 'nock';

test('getImgPokemonLogoJPEG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/imgs/pokemon_logo')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/jpeg');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getImgPokemonLogo('image/jpeg');
});

test('getImgPokemonLogoPNG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/imgs/pokemon_logo')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/png');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getImgPokemonLogo('image/png');
});

test('postImgPokemonLogoJPEG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/imgs/pokemon_logo')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('image/jpeg');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postImgPokemonLogo('image/jpeg', new Uint8Array());
});

test('postImgPokemonLogoPNG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/imgs/pokemon_logo')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('image/png');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postImgPokemonLogo('image/png', new Uint8Array());
});

test('getImgTownMapJPEG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/imgs/town_map')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/jpeg');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getImgTownMap('image/jpeg');
});

test('getImgTownMapPNG', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/imgs/town_map')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('image/png');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getImgTownMap('image/png');
});
