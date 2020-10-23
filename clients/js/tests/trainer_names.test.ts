import { Pkmnapi } from '../';
import nock from 'nock';

test('getTrainerNameAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/trainers/names')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTrainerNameAll();
});

test('getTrainerName', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/trainers/names/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTrainerName(0x01);
});

test('postTrainerName', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/trainers/names/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'trainer_names',
                    attributes: {
                        name: 'FOO',
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postTrainerName(0x01, {
        name: 'FOO',
    });
});
