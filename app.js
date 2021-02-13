class Snippet {
    constructor(el) {
        this.el = el;

        var langs = el.querySelectorAll('.snippet__lang');
        var codes = el.querySelectorAll('.snippet__code');

        this.langs = Array.prototype.slice.call(langs);
        this.codes = Array.prototype.slice.call(codes);
    }

    init() {
        var langs = this.langs;
        var codes = this.codes;

        langs.forEach(function (lang) {
            lang.addEventListener('click', function () {
                var lang_id = lang.getAttribute('data-lang');

                codes.forEach(function (code) {
                    var code_id = code.getAttribute('data-lang');

                    if (code_id === lang_id) {
                        lang.className = 'snippet__lang selected';
                        code.className = 'snippet__code';
                    } else {
                        code.className = 'snippet__code hidden';

                        langs.forEach(function (_lang) {
                            var _lang_id = _lang.getAttribute('data-lang');

                            if (_lang_id === lang_id) {
                                return;
                            }

                            _lang.className = 'snippet__lang';
                        });
                    }
                });
            });
        });
    }
}

var els = document.querySelectorAll('.snippet');

Array.prototype.slice.call(els).forEach(function (el) {
    var snippet = new Snippet(el);

    snippet.init();
});
