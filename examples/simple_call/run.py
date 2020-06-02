from flask import Flask, make_response
from lxml import etree

app = Flask(__name__)

@app.route("/")
@app.route("/index.html")
def index():
    html = open("index.html").read()
    return html

@app.route("/liquers_wasm_bg.wasm")
def liquers_wasm_bg_wasm():
    r = make_response(open("../../pkg/liquers_wasm_bg.wasm","rb").read())
    r.headers.set('Content-Type', "application/wasm")
    return r

@app.route("/liquers_wasm_bg.js")
def liquers_wasm_bg_js():
    r = make_response(open("../../pkg/liquers_wasm_bg.js","rb").read())
    r.headers.set('Content-Type', "text/javascript")
    return r

@app.route("/liquers_wasm.js")
def liquers_wasm_js():
    r = make_response(open("../../pkg/liquers_wasm.js","rb").read())
    r.headers.set('Content-Type', "text/javascript")
    return r

if __name__ == "__main__":
    app.run(debug=True)