import { Pkmnapi } from '../';
import nock from 'nock';

test('getTrainerPartiesAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/trainers/parties')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTrainerPartiesAll();
});

test('getTrainerParties', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/trainers/parties/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTrainerParties(0x01);
});

test('postTrainerParties', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/trainers/parties/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'trainer_parties',
                    attributes: {
                        parties: [
                            {
                                party: [
                                    {
                                        level: 10,
                                        pokemon: {
                                            id: '1',
                                        },
                                    },
                                ],
                            },
                        ],
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postTrainerParties(0x01, {
        parties: [
            {
                party: [
                    {
                        level: 10,
                        pokemon: {
                            id: '1',
                        },
                    },
                ],
            },
        ],
    });
});
