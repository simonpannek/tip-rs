export default function (sequelize, DataTypes) {
	return sequelize.define(
		'Guild',
		{
			id: { type: DataTypes.BIGINT, primaryKey: true },
			ignore: { type: DataTypes.BOOLEAN, allowNull: false, defaultValue: false }
		},
		{
			sequelize,
			tableName: 'guild',
			timestamps: false
		}
	);
}
