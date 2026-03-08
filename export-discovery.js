import { sources } from './sources.js';
import fs from 'fs';

const exportData = {
    providers: {},
    models: []
};

for (const [key, source] of Object.entries(sources)) {
    exportData.providers[key] = {
        name: source.name,
        url: source.url
    };
    for (const [id, label, tier, swe_score, ctx] of source.models) {
        exportData.models.push({
            id,
            label,
            tier,
            swe_score: parseFloat(swe_score.replace('%', '')),
            ctx,
            provider: key
        });
    }
}

fs.writeFileSync('../neuronclaw/discovery_data.json', JSON.stringify(exportData, null, 2));
console.log('✅ Discovery data exported to ../neuronclaw/discovery_data.json');
