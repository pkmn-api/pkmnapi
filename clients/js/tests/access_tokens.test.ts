import { Pkmnapi } from '../';
import nock from 'nock';

test('postAccessToken', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/access_tokens')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'access_tokens',
                    attributes: {
                        email_address: 'foo@bar.com',
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postAccessToken({
        email_address: 'foo@bar.com',
    });
});

test('postAccessTokenDelete', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/access_tokens/delete')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'access_tokens',
                    attributes: {
                        email_address: 'foo@bar.com',
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postAccessTokenDelete({
        email_address: 'foo@bar.com',
    });
});

test('deleteAccessToken', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .delete('/v1/access_tokens')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.deleteAccessToken({
        code: '1337',
        email_address: 'foo@bar.com',
    });
});
