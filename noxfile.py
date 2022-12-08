import nox


nox.options.reuse_existing_virtualenvs = True


@nox.session(python=["3.10", "3.11"])
def tests(session):
    args = session.posargs
    session.run("poetry", "install", external=True)
    session.run("maturin", "develop", "--release")
    session.run("pytest", *args)
