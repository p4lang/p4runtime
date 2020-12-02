import os
from setuptools import setup, find_packages
this_dir = os.path.dirname(os.path.realpath(__file__))
project_root = os.path.abspath(os.path.join(this_dir, os.pardir))

setup(
    name = "p4runtime",
    version = "1.3.0",
    packages = find_packages("."),
    install_requires = [
        "protobuf >= 3.6.1",
        "grpcio >= 1.17.2",
        "googleapis-common-protos >= 1.52"
    ],
    author = "P4 API Working Group",
    author_email = "p4-api@lists.p4.org",
    classifiers = [
        'License :: OSI Approved :: Apache Software License',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3',
    ],
    description = "The P4 Runtime protocol",
    long_description = open(project_root + "/README.md").read(),
    long_description_content_type = "text/markdown",
    license = "Apache-2.0",
    url = "https://github.com/p4lang/p4runtime"
)
