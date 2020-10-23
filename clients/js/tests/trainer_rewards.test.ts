import { Pkmnapi } from '../';
import nock from 'nock';

test('getTrainerRewardAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/trainers/rewards')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTrainerRewardAll();
});

test('getTrainerReward', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/trainers/rewards/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getTrainerReward(0x01);
});

test('postTrainerReward', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/trainers/rewards/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'trainer_rewards',
                    attributes: {
                        reward: 1337,
                    },
                },
            });
            expect(headers.accept).toBe('application/json');
            expect(headers['content-type']).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.postTrainerReward(0x01, {
        reward: 1337,
    });
});
