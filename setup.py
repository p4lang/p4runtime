from setuptools import setup, find_packages

setup(
    name = "p4runtime",
    version = "1.2.0",
    packages = find_packages("py"),
    install_requires = [
        "protobuf>=3.5.0",
        "grpcio >= 1.30",
        "googleapis-common-protos >= 1.52"
    ],
    package_dir = {'': 'py'},
    author = "P4 API Working Group",
    author_email = "p4-api@lists.p4.org",
    classifiers = [
        'License :: OSI Approved :: Apache Software License',
        'Programming Language :: Python',
        'Programming Language :: Python :: 2',
        'Programming Language :: Python :: 3',
    ],
    description = "The P4 Runtime protocol",
    long_description = open("README.md").read(),
    license = "Apache-2.0",
    url = "https://github.com/p4lang/p4runtime"
)
