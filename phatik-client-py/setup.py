
'''A setuptools based setup module.
See:
https://packaging.python.org/guides/distributing-packages-using-setuptools/
https://github.com/pypa/sampleproject
'''

from setuptools import setup
import pathlib

here = pathlib.Path(__file__).parent.resolve()

# Get the long description from the README file
long_description = (here / 'README.md').read_text(encoding='utf-8')

# Arguments marked as 'Required' below must be included for upload to PyPI.
# Fields marked as 'Optional' may be commented out.

setup(
    name='phatik-client',  # Required
    version='0.0.1',  # Required
    description='Python Client for Phatik',  # Optional

    long_description=long_description,  # Optional
    long_description_content_type='text/markdown',  # Optional (see note above)
    url='https://github.com/tgolsson/phatik',
    author='Tom Solberg',  # Optional
    author_email='me@sbg.dev',  # Optional

    classifiers=[
        'Development Status :: 3 - Alpha',
        'Intended Audience :: Developers',
        'Topic :: System :: Logging',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: 3 :: Only',
    ],
    keywords='events, logging, tracing, observability',
    packages=['.'],  # Required
    python_requires='>=3.7, <4',
    install_requires=['requests>=2'],  # Optional
    entry_points={  # Optional
        'console_scripts': [
            'phatik=phatik.cli:main',
        ],
    },

    project_urls={  # Optional
        'Bug Reports': 'https://github.com/tgolsson/phatik/issues',
        'Source': 'https://github.com/tgolsson/phatik/',
    },
)
