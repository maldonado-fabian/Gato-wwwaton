/** @type {import('next').NextConfig} */
import path from 'path';
const nextConfig = {
    webpack(config, options) {
        config.modules.rules.push({
            test: /\.scss$/,
            use: [
                options.defaultLoaders.babel,
                {
                    loader: 'sass-loader',
                    options: {
                        sassOptions: {
                            includePaths: [path.join(__dirname, 'styles')],
                        },
                    },
                },
            ],
        });

        return config;
    },
};

export default nextConfig;
