import { Pkmnapi } from '../';
import nock from 'nock';

test('getMartItemsAll', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/marts/items')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMartItemsAll();
});

test('getMartItems', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .get('/v1/marts/items/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toBe('');
            expect(headers.accept).toBe('application/json');

            done();

            return [200, '{}', {}];
        });

    pkmnapi.getMartItems(0x01);
});

test('postMartItems', (done) => {
    const pkmnapi = new Pkmnapi();

    nock('https://api.pkmnapi.com')
        .post('/v1/marts/items/1')
        .reply(function (_, body) {
            const { headers } = this.req;

            expect(body).toMatchObject({
                data: {
                    type: 'mart_items',
                    attributes: {
                        mart_items: [
                            {
                                id: '1',
                                type: 'item_names',
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

    pkmnapi.postMartItems(0x01, {
        mart_items: [
            {
                id: '1',
                type: 'item_names',
            },
        ],
    });
});
